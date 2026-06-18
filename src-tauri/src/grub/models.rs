use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Step {
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

impl From<modularitea_libs::infrastructure::grub::Step> for Step {
    fn from(step: modularitea_libs::infrastructure::grub::Step) -> Self {
        match step {
            modularitea_libs::infrastructure::grub::Step::CopyDir { from, to } => {
                Self::CopyDir { from, to }
            }
            modularitea_libs::infrastructure::grub::Step::CopyFile { from, to } => {
                Self::CopyFile { from, to }
            }
            modularitea_libs::infrastructure::grub::Step::SetGrubVar { key, value } => {
                Self::SetGrubVar { key, value }
            }
            modularitea_libs::infrastructure::grub::Step::ReplaceInFile {
                file,
                search,
                replace,
            } => Self::ReplaceInFile {
                file,
                search,
                replace,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ThemeManifest {
    pub name: String,
    pub version: String,
    pub github_url: Option<String>,
    pub preview_image: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub name_concat: Option<String>,
    pub steps: Vec<Step>,
}

impl From<modularitea_libs::infrastructure::grub::ThemeManifest> for ThemeManifest {
    fn from(manifest: modularitea_libs::infrastructure::grub::ThemeManifest) -> Self {
        Self {
            name: manifest.name,
            version: manifest.version,
            github_url: manifest.github_url,
            preview_image: manifest.preview_image,
            description: manifest.description,
            author: manifest.author,
            name_concat: manifest.name_concat,
            steps: manifest.steps.into_iter().map(Step::from).collect(),
        }
    }
}

