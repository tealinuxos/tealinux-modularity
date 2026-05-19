//! Reflector `--country` labels (Arch Linux). Only these may be passed through the UI.

pub const ALLOWED_REFLECTOR_COUNTRIES: &[&str] = &[
    "Indonesia",
    "Singapore",
    "Malaysia",
    "Thailand",
    "Vietnam",
    "Philippines",
    "Australia",
    "New Zealand",
    "United States",
    "Canada",
    "United Kingdom",
    "Germany",
    "France",
    "Netherlands",
    "Sweden",
    "Japan",
    "South Korea",
    "India",
];

pub fn is_allowed_mirror_country(candidate: &str) -> bool {
    let t = candidate.trim();
    ALLOWED_REFLECTOR_COUNTRIES
        .iter()
        .any(|c| c.eq_ignore_ascii_case(t))
}
