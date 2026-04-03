use modularitea_libs::error::{CommandErrorReturn, CommandOutput, ModulariteaError};

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LocalCommandError {
    pub operation: String,
    pub exit_code: Option<i32>,
    pub stderr: String,
}

impl From<CommandErrorReturn> for LocalCommandError {
    fn from(err: CommandErrorReturn) -> Self {
        Self {
            operation: err.operation,
            exit_code: err.exit_code,
            stderr: err.stderr,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "data")]
pub enum LocalModulariteaError {
    ProfileReadError {
        path: String,
        source: String,
    },
    ProfileParseError {
        path: String,
        source: String,
    },
    ProfileValidationError {
        message: String,
    },

    PlanningError {
        message: String,
    },
    DependencyError {
        message: String,
    },
    CircularDependencyError {
        cycle: String,
    },

    ExecutionError {
        task_name: String,
        source: String,
    },
    RollbackError {
        task_name: String,
        reason: String,
    },

    PacmanError(LocalCommandError),
    GrubError {
        operation: String,
        reason: String,
    },
    SystemctlError {
        operation: String,
        exit_code: Option<i32>,
        stderr: String,
    },
    FilesystemError {
        operation: String,
        source: String,
    },

    PrivilegeError {
        reason: String,
    },
    PkexecNotFound,
    PolkitCancelled,
    RootBinaryNotFound {
        binary: String,
    },

    CommandError {
        command: String,
        exit_code: Option<i32>,
        stderr: String,
    },
    IoError(String),
    InternalError(String),
}

impl From<ModulariteaError> for LocalModulariteaError {
    fn from(err: ModulariteaError) -> Self {
        match err {
            ModulariteaError::ProfileReadError { path, source } => Self::ProfileReadError {
                path: path.to_string_lossy().into_owned(),
                source: source.to_string(),
            },
            ModulariteaError::ProfileParseError { path, source } => Self::ProfileParseError {
                path: path.to_string_lossy().into_owned(),
                source: source.to_string(),
            },
            ModulariteaError::ProfileValidationError { message } => {
                Self::ProfileValidationError { message }
            }

            ModulariteaError::PlanningError { message } => Self::PlanningError { message },
            ModulariteaError::DependencyError { message } => Self::DependencyError { message },
            ModulariteaError::CircularDependencyError { cycle } => {
                Self::CircularDependencyError { cycle }
            }

            ModulariteaError::ExecutionError { task_name, source } => Self::ExecutionError {
                task_name,
                source: source.to_string(),
            },
            ModulariteaError::RollbackError { task_name, reason } => {
                Self::RollbackError { task_name, reason }
            }

            ModulariteaError::PacmanError(cmd_err) => Self::PacmanError(cmd_err.into()),
            ModulariteaError::GrubError { operation, reason } => {
                Self::GrubError { operation, reason }
            }
            ModulariteaError::SystemctlError {
                operation,
                exit_code,
                stderr,
            } => Self::SystemctlError {
                operation,
                exit_code,
                stderr,
            },
            ModulariteaError::FilesystemError { operation, source } => Self::FilesystemError {
                operation,
                source: source.to_string(),
            },

            ModulariteaError::PrivilegeError { reason } => Self::PrivilegeError { reason },
            ModulariteaError::PkexecNotFound => Self::PkexecNotFound,
            ModulariteaError::PolkitCancelled => Self::PolkitCancelled,
            ModulariteaError::RootBinaryNotFound { binary } => Self::RootBinaryNotFound { binary },

            ModulariteaError::CommandError {
                command,
                exit_code,
                stderr,
            } => Self::CommandError {
                command,
                exit_code,
                stderr,
            },
            ModulariteaError::IoError(e) => Self::IoError(e.to_string()),
            ModulariteaError::InternalError(s) => Self::InternalError(s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct LocalCommandOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl From<CommandOutput> for LocalCommandOutput {
    fn from(output: CommandOutput) -> Self {
        Self {
            success: output.success(),
            exit_code: output.exit_code,
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}
