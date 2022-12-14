pub mod Path {
    use std::path::PathBuf;
    use tauri::api::path::download_dir;

    static HOME_NAME: &'static str = "Pic Collector";

    pub fn base_path() -> PathBuf {
        let base_path = download_dir().unwrap().join(HOME_NAME);
        if !base_path.exists() {
            std::fs::create_dir(&base_path).unwrap();
        }

        base_path
    }

}