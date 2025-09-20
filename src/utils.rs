#[inline]
pub fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

#[inline]
pub fn normalize_label(label: &str) -> &str {
    match label {
        "iwlwifi_1 temp1" => "Wi-Fi Module",
        "sensor 1"        => "Sensor 1",
        "sensor 2"        => "Sensor 2",
        "composite"       => "Chipset",
        "edge"            => "GPU (Edge)",
        "tctl"            => "CPU (Tctl)",
        _ => label,
    }
}