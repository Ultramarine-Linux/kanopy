use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::helm::HelmChart;

/// Randomly generate a new token
/// A token format is "[a-z0-9]{6}.[a-z0-9]{16}"
pub fn random_token() -> String {
    // pubkey: 6 chars
    // privkey: 16 chars
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let pubkey: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(6)
        .collect();
    let privkey: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(16)
        .collect();
    format!("{}.{}", pubkey, privkey).to_lowercase()
}

// default is Worker
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum NodeRole {
    Master,
    Worker,
}

impl Default for NodeRole {
    fn default() -> Self {
        NodeRole::Worker
    }
}

fn default_as_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct JoinParams {
    /// Join this node to database cluster
    /// Only works if role is ControlPlane and kine is not enabled
    pub endpoint: Option<String>,
    /// Cluster joining token
    /// If not provided, a random token will be generated
    /// A token format is "[a-z0-9]{6}.[a-z0-9]{16}"
    /// Should always be provided if joining to an existing cluster
    #[serde(default = "random_token")]
    pub token: String,
    pub ca_cert: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct ClusterConfig {
    /// Node is master (first node)
    pub master: bool,
    /// Role of node
    pub role: NodeRole,
    /// Join this node to database cluster
    /// Only works if role is ControlPlane and kine is not enabled
    #[serde(default = "default_as_true")]
    pub run_db: bool,
    /// Cluster joining parameters
    pub join: JoinParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", default)]
pub struct NetworkingConfig {
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
    pub cni: String,
    pub pod_cidr: Option<String>,
    pub service_cidr: Option<String>,
    pub cni_values: Option<Value>,
}

impl Default for NetworkingConfig {
    fn default() -> Self {
        NetworkingConfig {
            internal_ip: None,
            external_ip: None,
            cni: "flannel".to_string(),
            pod_cidr: Some("10.42.0.0/16".to_string()),
            service_cidr: None,
            cni_values: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct KineConfig {
    pub enabled: bool,
    pub datastore: Option<String>,
    pub ca_cert: Option<String>,
    pub cert: Option<String>,
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct DatabaseConfig {
    pub kine: KineConfig,
    pub etcd_args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct HelmConfig {
    pub extra_args: Vec<String>,
    pub extra_charts: Vec<HelmChart>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct KanopyConfig {
    pub hostname: String,
    pub cluster: ClusterConfig,
    pub networking: NetworkingConfig,
    pub database: DatabaseConfig,
    pub helm: HelmConfig,
}

impl KanopyConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        // load from file, missing keys will be default
        let file = std::fs::read_to_string(path)?;
        let config: KanopyConfig = serde_yaml::from_str(&file)?;
        Ok(config)
    }
}
