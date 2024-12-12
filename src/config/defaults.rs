use std::path::PathBuf;

pub(super) fn default_project_name() -> String {
    // Infer from the current directory
    std::env::current_dir()
        .expect("Failed to get current directory")
        .file_name()
        .expect("Failed to get current directory name")
        .to_string_lossy()
        .to_string()
}

pub(super) fn default_cache_dir() -> PathBuf {
    let mut path = dirs::cache_dir().expect("Failed to get cache directory");
    path.push("kwaak");
    path
}

pub(super) fn default_log_dir() -> PathBuf {
    let mut path = dirs::cache_dir().expect("Failed to get cache directory");
    path.push("kwaak");
    path.push("logs");

    path
}

pub(super) fn default_dockerfile() -> PathBuf {
    "./Dockerfile".into()
}

pub(super) fn default_docker_context() -> PathBuf {
    ".".into()
}

pub(super) fn default_main_branch() -> String {
    "main".to_string()
}
