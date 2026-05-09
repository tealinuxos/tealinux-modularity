//! Primary display geometry from Fastfetch (`Display`), used for GRUB `GRUB_GFXMODE`.

use crate::sysinfo::{
    types::display_fastfetch::DisplayFastfetch,
    utils::fetch_module::fetch_module,
};

/// Primary display or [`resolution`] crate fallback (for GRUB `GRUB_GFXMODE`).
pub fn grub_screen_resolution_px() -> (u32, u32) {
    primary_display_px()
        .or_else(|| {
            resolution::current_resolution()
                .ok()
                .map(|(w, h)| (w as u32, h as u32))
        })
        .unwrap_or((1920, 1080))
}

pub fn primary_display_px() -> Option<(u32, u32)> {
    let display_vec: DisplayFastfetch = fetch_module("Display");
    display_vec
        .into_iter()
        .flat_map(|block| block.result.into_iter())
        .find(|d| d.primary)
        .map(|d| {
            (
                (d.output.width.max(1) as u64).clamp(1, u64::from(u32::MAX)) as u32,
                (d.output.height.max(1) as u64).clamp(1, u64::from(u32::MAX)) as u32,
            )
        })
}
