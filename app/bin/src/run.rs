use color_eyre::eyre::Result;
use std::process::Command;

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
        print!("Checking for {}... ", command);
        let mut cmd = Command::new("which");
        cmd.arg(command);
        let output = cmd.output()?;
        if !output.status.success() {
            println!("{} not found", command);
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

pub fn init_cluster() -> Result<()> {
    let mut binding = Command::new("kubeadm").arg("init").spawn()?;
    binding.wait()?;

    Ok(())
}
