use modularitea_libs::infrastructure::grub::{GrubInstruction, GrubInstructionExecutor};
use resolution::current_resolution;

pub struct GrubManager {
    pub instruction: GrubInstruction,
}

impl GrubManager {
    pub fn new() -> Self {
        let resolution = current_resolution().unwrap_or((1920, 1080));

        // let instruction = GrubInstruction::new()
        //     .set_screen_resolution(resolution.0 as u32, resolution.1 as u32);
        let instruction = GrubInstruction::with_themes_dir(
            "/usr/share/modularitea-libs/grub-theme"
                .to_string(),
        )
        .set_screen_resolution(resolution.0 as u32, resolution.1 as u32);

        Self { instruction }
    }
}
