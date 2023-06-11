//! Kanopy addons

use serde::{Deserialize, Serialize};
use color_eyre::Result;




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

        let kustomize_path = if std::path::Path::new(&format!("{}/kustomize/kustomization.yaml", path)).exists() {
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