use crate::grub::executor::{GrubInstruction, GrubInstructionExecutor};
use crate::sysinfo::display_resolution::grub_screen_resolution_px;

pub struct GrubManager {
    pub instruction: GrubInstruction,
}

impl GrubManager {
    pub fn new() -> Self {
        let (w, h) = grub_screen_resolution_px();

        let instruction = GrubInstruction::new().set_screen_resolution(w, h);

        Self { instruction }
    }
}
