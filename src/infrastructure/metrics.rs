use sysinfo::{System, Components, Disks};

use crate::domain::entities::{HostStats, SystemDetails, DiskInfo,  ComponentInfo, MemoryInfo};
use crate::utils::{round2, normalize_label};

pub fn get_info() -> HostStats {
    let mut sys:System = System::new_all();
    sys.refresh_memory();

    let mut disks = Disks::new_with_refreshed_list();
    disks.refresh(true);
    let mut components = Components::new_with_refreshed_list();
    components.refresh(true);

    let system: SystemDetails = SystemDetails {
        name: System::name().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
    };

    let total_kib = sys.total_memory() as u64;
    let used_kib  = sys.used_memory() as u64;
    let total_b   = total_kib.saturating_mul(1024);
    let used_b    = used_kib.saturating_mul(1024);

    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;

    let total_gib = (total_b as f64) / GIB;
    let used_gib  = (used_b  as f64) / GIB;
    let total_mib = (total_b as f64) / MIB;
    let used_mib  = (used_b  as f64) / MIB;

    let swap_total_kib = sys.total_swap() as u64;
    let swap_used_kib  = sys.used_swap() as u64;
    let swap_total_b   = swap_total_kib.saturating_mul(1024);
    let swap_used_b    = swap_used_kib.saturating_mul(1024);

    let swap_total_gib = (swap_total_b as f64) / GIB;
    let swap_used_gib  = (swap_used_b  as f64) / GIB;
    let swap_total_mib = (swap_total_b as f64) / MIB;
    let swap_used_mib  = (swap_used_b  as f64) / MIB;

    let ram_percent  = if total_b > 0 { (used_b as f64 / total_b as f64) * 100.0 } else { 0.0 };
    let swap_percent = if swap_total_b > 0 { (swap_used_b as f64 / swap_total_b as f64) * 100.0 } else { 0.0 };

    let memory = MemoryInfo {
        total_ram_gb: round2(total_gib),
        total_ram_mb: round2(total_mib),
        used_ram_gb:  round2(used_gib),
        used_ram_mb:  round2(used_mib),
        ram_percent:  round2(ram_percent),

        total_swap_gb: round2(swap_total_gib),
        total_swap_mb: round2(swap_total_mib),
        used_swap_gb:  round2(swap_used_gib),
        used_swap_mb:  round2(swap_used_mib),
        swap_percent:  round2(swap_percent),
    };

    let mut disks_info = Vec::with_capacity(disks.iter().count());
    for d in disks.iter() {
        let total_b     = d.total_space() as u128;   
        let available_b = d.available_space() as u128; 
        let total_gib   = (total_b as f64) / GIB;
        let total_mib   = (total_b as f64) / MIB;
        let avail_gib   = (available_b as f64) / GIB;
        let avail_mib   = (available_b as f64) / MIB;

        disks_info.push(DiskInfo {
            name: d.name().to_string_lossy().into_owned(),
            mount_point: d.mount_point().to_string_lossy().into_owned(),
            available_space_gb: round2(avail_gib),
            available_space_mb: round2(avail_mib),
            total_space_gb:     round2(total_gib),
            total_space_mb:     round2(total_mib),
        });
    }

    let mut components_info = Vec::with_capacity(components.iter().count());
    for c in components.iter() {
        components_info.push(ComponentInfo {
            label: normalize_label(c.label()).to_string(),
            temperature: c.temperature(),
        });
    }

    HostStats { system, memory, disks: disks_info, components: components_info }
}