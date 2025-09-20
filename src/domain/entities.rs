use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemDetails {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub available_space_gb: f64,
    pub available_space_mb: f64,
    pub total_space_gb: f64,
    pub total_space_mb: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentInfo {
    pub label: String,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoryInfo {
    pub total_ram_gb: f64,
    pub total_ram_mb: f64,
    pub used_ram_gb: f64,
    pub used_ram_mb: f64,
    pub ram_percent: f64,
    pub total_swap_gb: f64,
    pub total_swap_mb: f64,
    pub used_swap_gb: f64,
    pub used_swap_mb: f64,
    pub swap_percent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HostStats {
    pub system: SystemDetails,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub components: Vec<ComponentInfo>,
}