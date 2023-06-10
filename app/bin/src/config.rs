use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

// default is Worker
#[derive(Serialize, Deserialize, Debug)]
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
pub struct JoinParams {
    /// Join this node to database cluster
    /// Only works if role is ControlPlane and kine is not enabled
    endpoint: Option<String>,
    token: Option<String>,
    ca_cert: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkingConfig {
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
    pub cni: Option<String>,
    pub pod_cidr: Option<String>,
    pub service_cidr: Option<String>,
    pub cni_values: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KineConfig {
    pub enabled: bool,
    pub datastore: Option<String>,
    pub ca_cert: Option<String>,
    pub cert: Option<String>,
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DatabaseConfig {
    pub kine: KineConfig,
    pub etcd_args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KanopyConfig {
    pub hostname: String,
    pub cluster: ClusterConfig,
    pub networking: NetworkingConfig,
    pub database: DatabaseConfig,
}

impl KanopyConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        // load from file, missing keys will be default
        let file = std::fs::read_to_string(path)?;
        let config: KanopyConfig = serde_yaml::from_str(&file)?;
        Ok(config)
    }
}
