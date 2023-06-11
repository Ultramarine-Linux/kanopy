// Helm CLI wrapper for kanopy

use std::{fs::File, io::Write, process::Command, path::PathBuf};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tempfile::TempPath;
use tracing::info;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HelmChart {
    pub name: String,
    pub version: Option<String>,
    pub repo: String,
    pub namespace: String,
    pub values: Option<Value>,
    pub values_file: Option<String>,
}

impl HelmChart {
    pub fn new(
        name: String,
        version: Option<String>,
        repo: String,
        namespace: String,
        values: Option<Value>,
        values_file: Option<String>,
    ) -> HelmChart {
        HelmChart {
            name,
            version,
            repo,
            namespace,
            values,
            values_file,
        }
    }
    pub fn read_from_file(path: &str) -> Result<Self> {
        info!("Reading chart from file: {}", path);
        let file = File::open(path)?;
        let chart: HelmChart = serde_yaml::from_reader(file)?;
        Ok(chart)
    }
    /// Generate values file for helm install
    /// returns temporary file path to values file
    pub fn generate_values(&self) -> Result<Option<PathBuf>> {
        println!("Generating values file for chart: {:?}", self.values);
        if self.values.is_none() {
            return Ok(None);
        } else {
            let (mut tmpfile, path) = tempfile::Builder::new()
                .prefix(&format!("{}", self.name))
                .suffix(".yaml")
                .tempfile()?.keep()?;
            let values = serde_yaml::to_string(&self.values)?;
            tmpfile.write_all(values.as_bytes())?;

            Ok(Some(path))
        }
    }
    pub fn generate_args(&self) -> Result<Vec<String>> {
        let extra_values_file = self.generate_values()?;

        let mut args = vec![self.name.clone(), self.name.clone()];
        if let Some(version) = &self.version {
            args.push("--version".to_string());
            args.push(version.to_string());
        }
        args.push("--repo".to_string());
        args.push(self.repo.clone());
        args.push("--namespace".to_string());
        args.push(self.namespace.clone());
        if self.values_file.is_some() {
            args.push("--values".to_string());
            args.push(self.values_file.clone().unwrap());
        }
        // extra values
        if extra_values_file.is_some() {
            args.push("--values".to_string());
            args.push(extra_values_file.unwrap().to_str().unwrap().to_string());
        }
        Ok(args)
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HelmInstaller {
    pub kubeconfig: String,
    pub extra_args: Vec<String>,
    pub chart: HelmChart,
}

/// Reads a helm chart from a folder in kanopy format
pub fn chart_from_folder(path: &str, values: Option<Value>) -> Result<HelmChart> {
    let mut chart = HelmChart::read_from_file(&format!("{}/chart.yaml", path))?;
    let values_file = format!("{}/values.yaml", path);
    chart.values_file = Some(values_file);
    chart.values = values;
    Ok(chart)
}

// generate args for helm install command

impl HelmInstaller {
    pub fn new(kubeconfig: String, extra_args: Vec<String>, chart: HelmChart) -> HelmInstaller {
        HelmInstaller {
            kubeconfig,
            extra_args,
            chart,
        }
    }

    pub fn install(&self) -> Result<()> {
        let mut cmd = Command::new("helm")
            .arg("install")
            .arg("--kubeconfig")
            .arg(&self.kubeconfig)
            .args(self.chart.generate_args()?)
            .args(self.extra_args.clone())
            .spawn()?.wait()?;

        // run command
        if cmd.success() {
            println!("Helm install successful for chart: {}", self.chart.name);
            Ok(())
        } else {
            println!("Helm install failed! {}", cmd);
            Err(color_eyre::eyre::eyre!("Helm install failed"))
        }
    }
}
