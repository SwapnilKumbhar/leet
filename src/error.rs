use thiserror::Error;

//TODO: This could be better

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to parse config.yaml: {}", .msg)]
    YamlParseError { msg: String },

    #[error("OS Error: {}", .msg)]
    OsError { msg: String },

    #[error("OS String Error")]
    OsStringError {},

    #[error("Failed to open file: {}", .file_name)]
    FileOpenError { file_name: String },

    #[error("Config not found in the default paths.")]
    ConfigNotFoundError {},

    #[error("Environment variable not found: {}", .var)]
    EnvNotFoundError { var: String },
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Failed to copy `{}` to `{}`", .src, .dst)]
    CopyError { src: String, dst: String },

    #[error("OS Error.")]
    OsError(#[from] std::io::Error),

    #[error("OS String Error")]
    OsStringError {},

    #[error("Error: {}", .msg)]
    GenericError { msg: String },

    #[error("Directory already exists: {}", .dir_name)]
    DirectoryExistsError { dir_name: String },
}