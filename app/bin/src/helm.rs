// Helm CLI wrapper for kanopy

use std::{fs::File, io::Write, process::Command};

use color_eyre::Result;
use serde_yaml::Value;
use tempfile::TempPath;

pub struct HelmChart {
    pub name: String,
    pub version: Option<String>,
    pub repo: String,
    pub namespace: String,
    pub values: Option<Value>,
    pub values_file: String,
}

impl HelmChart {
    pub fn new(
        name: String,
        version: Option<String>,
        repo: String,
        namespace: String,
        values: Option<Value>,
        values_file: String,
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
    /// Generate values file for helm install
    /// returns temporary file path to values file
    pub fn generate_values(&self) -> Result<Option<TempPath>> {
        if self.values.is_none() {
            return Ok(None);
        } else {
            let mut tmpfile = tempfile::Builder::new()
                .prefix(&format!("{}", self.name))
                .suffix(".yaml")
                .tempfile()?;
            let values = serde_yaml::to_string(&self.values)?;
            tmpfile.write_all(values.as_bytes())?;

            Ok(Some(tmpfile.into_temp_path()))
        }
    }
    pub fn generate_args(&self) -> Result<Vec<String>> {
        let extra_values_file = self.generate_values()?;

        let mut args = vec!["--name".to_string(), self.name.clone()];
        if let Some(version) = &self.version {
            args.push("--version".to_string());
            args.push(version.to_string());
        }
        args.push("--repo".to_string());
        args.push(self.repo.clone());
        args.push("--namespace".to_string());
        args.push(self.namespace.clone());
        args.push("--values".to_string());
        args.push(self.values_file.clone());
        // extra values
        if extra_values_file.is_some() {
            args.push("--values".to_string());
            args.push(extra_values_file.unwrap().to_str().unwrap().to_string());
        }
        Ok(args)
    }
}

pub struct HelmInstaller {
    pub kubeconfig: String,
    pub extra_args: Vec<String>,
    pub chart: HelmChart,
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
            .output()?;

        // run command
        if cmd.status.success() {
            println!("Helm install successful for chart: {}", self.chart.name);
            Ok(())
        } else {
            println!("Helm install failed");
            Err(color_eyre::eyre::eyre!("Helm install failed"))
        }
    }
}
