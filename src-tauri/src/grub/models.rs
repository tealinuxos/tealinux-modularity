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
