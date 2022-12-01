pub mod Path {
    use std::path::PathBuf;
    use tauri::api::path::download_dir;

    static HOME_NAME: &'static str = "Pic Collector";

    pub fn base_path() -> PathBuf {
        download_dir().unwrap().join(HOME_NAME)
    }

}