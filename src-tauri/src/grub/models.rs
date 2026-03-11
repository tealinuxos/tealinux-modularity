use modularitea_libs::infrastructure::grub::{Step as LibStep, ThemeManifest as LibThemeManifest};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LocalStep {
    CopyDir {
        from: String,
        to: String,
    },
    CopyFile {
        from: String,
        to: String,
    },
    SetGrubVar {
        key: String,
        value: String,
    },
    ReplaceInFile {
        file: String,
        search: String,
        replace: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LocalThemeManifest {
    pub name: String,
    pub version: String,
    pub github_url: Option<String>,
    pub preview_image: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub name_concat: Option<String>,
    pub steps: Vec<LocalStep>,
}

impl From<LibStep> for LocalStep {
    fn from(value: LibStep) -> Self {
        match value {
            LibStep::CopyDir { from, to } => LocalStep::CopyDir { from, to },
            LibStep::CopyFile { from, to } => LocalStep::CopyFile { from, to },
            LibStep::SetGrubVar { key, value } => LocalStep::SetGrubVar { key, value },
            LibStep::ReplaceInFile {
                file,
                search,
                replace,
            } => LocalStep::ReplaceInFile {
                file,
                search,
                replace,
            },
        }
    }
}

impl From<LibThemeManifest> for LocalThemeManifest {
    fn from(value: LibThemeManifest) -> Self {
        LocalThemeManifest {
            name: value.name,
            version: value.version,
            github_url: value.github_url,
            preview_image: value.preview_image,
            description: value.description,
            author: value.author,
            name_concat: value.name_concat,
            steps: value.steps.into_iter().map(LocalStep::from).collect(),
        }
    }
}
