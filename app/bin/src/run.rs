use color_eyre::eyre::Result;
use serde_json::json;
use std::{
    fs::create_dir_all,
    io::Write,
    process::{Command, Stdio},
};
use templar::{Context, StandardContext, Templar};
use tracing::info;

use crate::{
    config::KanopyConfig,
    helm::{chart_from_folder, HelmInstaller},
    KanopyCli,
};

// required commands

const REQUIRED_COMMANDS: &[&str] = &[
    "kubeadm",
    "kubectl",
    "helm",
    "crio",
    "git",
    "curl",
    "tar",
    "systemctl",
];

pub fn check_commands() -> Result<()> {
    for command in REQUIRED_COMMANDS {
        info!("Checking for {}... ", command);
        let mut cmd = Command::new("which");
        cmd.arg(command);
        let output = cmd.output()?;
        if !output.status.success() {
            tracing::log::error!("{} not found", command);
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Enable services
pub fn enable_services() -> Result<()> {
    let mut cmd = Command::new("systemctl");
    cmd.arg("enable").arg("--now").arg("crio").arg("kubelet");
    let output = cmd.output()?;
    if !output.status.success() {
        println!("Failed to enable services");
        std::process::exit(1);
    }
    Ok(())
}

pub fn render_kubeadm_init(config: &KanopyConfig) -> Result<String> {
    let cluster_configuration = json!({
        "apiVersion": "kubeadm.k8s.io/v1beta3",
        "kind": "ClusterConfiguration",
        "kubernetesVersion": "v1.26.0",
        "controllerManager": {
            "extraArgs": {
                "flex-volume-plugin-dir": "/etc/kubernetes/kubelet-plugins/volume/exec"
            }
        },
        "networking": {
            "podSubnet": config.networking.pod_cidr,
            "serviceSubnet": config.networking.service_cidr
        },
    });
    let init_configuration = json!({
        "apiVersion": "kubeadm.k8s.io/v1beta3",
        "kind": "InitConfiguration",
        "bootstrapTokens": [
            {
                "groups" : [
                    "system:bootstrappers:kubeadm:default-node-token"
                ],
                "token": config.cluster.join.token,
                "ttl": "24h0m0s",
                "usages": [
                    "signing",
                    "authentication"
                ]
            },
        ],
        "nodeRegistration": {
            "name": config.hostname,
            "criSocket": "/var/run/crio/crio.sock",
            "taints": [],
            "kubeletExtraArgs": {
                "cgroup-driver": "systemd",
                "feature-gates": "NodeSwap=true"
            }
        }
    });

    Ok(format!(
        "{}\n---\n{}",
        serde_yaml::to_string(&cluster_configuration)?,
        serde_yaml::to_string(&init_configuration)?
    ))
}

pub fn render_join_template(config: &KanopyConfig) -> Result<String> {
    let join_configuration = json!({
        "apiVersion": "kubeadm.k8s.io/v1beta3",
        "kind": "JoinConfiguration",
        "discovery": {
            "bootstrapToken": {
                "apiServerEndpoint": config.cluster.join.endpoint,
                "token": config.cluster.join.token,
                "unsafeSkipCAVerification": true
            }
        },
        "nodeRegistration": {
            "criSocket": "/var/run/crio/crio.sock",
            "name": config.hostname,
            "taints": [],
        }
    });

    Ok(serde_yaml::to_string(&join_configuration)?)
}
pub fn install_cni(params: KanopyCli, config: KanopyConfig) -> Result<()> {
    // get kubeconfig from /etc/kubernetes/admin.conf
    info!("Installing CNI `{}`...", config.networking.cni);
    let kubeconfig = "/etc/kubernetes/admin.conf";
    let chart_path = format!("{}/cnis/{}", params.assets, config.networking.cni);
    let chart = chart_from_folder(&chart_path, config.networking.cni_values)?;
    let installer = HelmInstaller::new(kubeconfig.to_owned(), Vec::new(), chart);
    installer.install()?;
    Ok(())
}

pub fn init_cluster(params: KanopyCli) -> Result<()> {
    info!("Initializing Cluster");
    let config = KanopyConfig::load_from_file(&params.config)?;

    create_dir_all("/etc/kanopy")?;

    match config.cluster.role {
        crate::config::NodeRole::Master => {
            info!("Initializing Master");
            let rendered = render_kubeadm_init(&config)?;
            println!("{}", rendered);
            // write file to /etc/kanopy/kubeadm.yaml
            let mut file = std::fs::File::create("/etc/kanopy/kubeadm.yaml")?;
            file.write_all(rendered.as_bytes())?;
            Command::new("kubeadm")
                .arg("init")
                .arg("--config")
                .arg("/etc/kanopy/kubeadm.yaml")
                .arg("-v=5")
                .spawn()?
                .wait()?;
            info!("Installing CNI");
            install_cni(params, config)?;
        }
        crate::config::NodeRole::Worker => {
            info!("Initializing Worker");
            let rendered = render_join_template(&config)?;
            println!("{}", rendered);
            let mut file = std::fs::File::create("/etc/kanopy/kubeadm.yaml")?;
            file.write_all(rendered.as_bytes())?;
            Command::new("kubeadm")
                .arg("join")
                .arg("--config")
                .arg("/etc/kanopy/kubeadm.yaml")
                .spawn()?
                .wait()?;
        }
    }

    Ok(())
}
