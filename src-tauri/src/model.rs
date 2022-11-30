use serde_with::serde_as;

#[serde_as]
#[derive(serde::Serialize)]
pub struct ImagesFolderInfoFile {
    pub is_dir: bool,
    pub path: String,
    pub name: String,
    pub files: Vec<String>,
}

#[serde_as]
#[derive(serde::Serialize)]
pub struct ImagesFolderInfo {
    pub name: String,
    pub files: Vec<ImagesFolderInfoFile>, // 2度文件目录
}