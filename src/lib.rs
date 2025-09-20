pub mod domain;
pub mod presentation;
pub mod infrastructure;
pub mod utils;

pub use domain::entities::{HostStats,SystemDetails, DiskInfo,  ComponentInfo, MemoryInfo};

pub use presentation::cli::Cli;
pub use presentation::api::start_server;

pub use infrastructure::metrics;

pub use utils::{round2, normalize_label};