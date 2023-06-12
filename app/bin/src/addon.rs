//! Kanopy addons

use std::process::Command;

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tracing::info;

use crate::helm::{HelmInstaller, chart_from_folder};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Addon {
    pub name: String,
    // Optional path to kustomize folder, absolute and will
    // be loaded if path exists
    pub kustomize: Option<String>,
    // Optional path to helm chart, is path to the whole addon
    pub helm: Option<String>,
}

impl Addon {
    pub fn load_from_dir(path: &str) -> Result<Self> {
        // get folder name from path
        let name = path.split("/").last().unwrap().to_string();

        // if chart.yaml exists, load helm chart
        let chart_path = if std::path::Path::new(&format!("{}/chart.yaml", path)).exists() {
            Some(path.to_string())
        } else {
            None
        };

        // if kustomization.yaml exists, load kustomize path subfolder

        let kustomize_path =
            if std::path::Path::new(&format!("{}/kustomize/kustomization.yaml", path)).exists() {
                Some(format!("{}/kustomize", path))
            } else {
                None
            };

        Ok(Addon {
            name,
            helm: chart_path,
            kustomize: kustomize_path,
        })
    }
}

pub struct AddonLoader {
    pub kubeconfig: String,
}

impl AddonLoader {
    pub fn new(kubeconfig: String) -> AddonLoader {
        AddonLoader { kubeconfig }
    }

    pub fn load_kustomize(&self, path: &str) -> Result<()> {
        info!("Installing Kustomize from path: {}", path);
        let cmd = Command::new("kubectl")
            .arg("apply")
            .arg("-k")
            .arg(path)
            .env("KUBECONFIG", &self.kubeconfig)
            .spawn()?.wait()?;
        if cmd.success() {
            println!("Kustomize install successful for path: {}", path);
        } else {
            println!("Kustomize install failed! {}", cmd);
            return Err(color_eyre::eyre::eyre!("Kustomize install failed"));
        }
        Ok(())
    }

    pub fn load_helm(&self, path: &str, values: Option<Value>) -> Result<()> {
        info!("Installing Helm chart from path: {}", path);
        let helm_installer = HelmInstaller::new(self.kubeconfig.clone(), vec![], chart_from_folder(path, values)?);
        helm_installer.install()?;
        Ok(())
    }

    pub fn load(&self, addon: &Addon, values: Option<Value>) -> Result<()> {
        info!("Loading addon: {}", addon.name);
        if let Some(kustomize_path) = &addon.kustomize {
            self.load_kustomize(kustomize_path)?;
        }
        if let Some(helm_path) = &addon.helm {
            self.load_helm(helm_path, values)?;
        }
        Ok(())
    }
}

pub fn load_addons_dir(path: &str) -> Result<Vec<Addon>> {
    // list of addons to return
    let mut addons: Vec<Addon> = Vec::new();

    // for each folder in addons_path, load addon, and make path absolute
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let addon = Addon::load_from_dir(&path.to_string_lossy())?;
        addons.push(addon);
    }

    Ok(addons)
}

/// Load a list of CNI addons
pub fn load_cni_addons(asset_path: &str) -> Result<Vec<Addon>> {
    // list of addons to return
    let cni_path = format!("{}/cnis", asset_path);
    load_addons_dir(&cni_path)
}

/// load general addons
pub fn load_addons(asset_path: &str) -> Result<Vec<Addon>> {
    // list of addons to return
    let general_path = format!("{}/addons", asset_path);
    load_addons_dir(&general_path)
}
