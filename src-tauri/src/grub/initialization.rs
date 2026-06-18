use modularitea_libs::infrastructure::grub::{GrubInstruction, GrubInstructionExecutor};

use crate::sysinfo::display_resolution::grub_screen_resolution_px;
use crate::grub::executor::grub_themes_dir_path;

pub struct GrubManager {
    pub instruction: GrubInstruction,
}

impl GrubManager {
    pub fn new() -> Self {
        let (w, h) = grub_screen_resolution_px();
        let themes_dir = grub_themes_dir_path();

        eprintln!("[GrubManager] Using themes dir: {}", themes_dir);

        let instruction = GrubInstruction::with_themes_dir(themes_dir)
            .set_screen_resolution(w, h);

        eprintln!("[GrubManager] Loaded {} themes", instruction.manifest.len());

        Self { instruction }
    }
}
