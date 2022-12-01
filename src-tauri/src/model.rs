use serde_with::serde_as;

use crate::dao::imageview_dao::Pagination;

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
#[serde_as]
#[derive(serde::Serialize)]
pub struct FileMetaList {
    pub list: Vec<FileMeta>,
    pub pagination: Pagination,
}

#[serde_as]
#[derive(serde::Serialize)]
pub struct FileMeta {
    id: i64,
    path: String,
    orign_name: String,
    name: String,
    type_name: String,
    length: i64,
    create_time: i64,
    update_time: i64,
}

impl FileMeta {
    pub fn from(row: &[sqlite::Value]) -> Self {
        // println!("row {:?}", row);
        FileMeta {
            id: row[0].as_integer().unwrap(),
            path: String::from(row[1].as_string().unwrap()),
            orign_name: String::from(row[2].as_string().unwrap()),
            name: String::from(row[3].as_string().unwrap()),
            type_name: String::from(row[4].as_string().unwrap()),
            length: row[5].as_integer().unwrap(),
            create_time: row[6].as_integer().unwrap(),
            update_time: row[7].as_integer().unwrap(),
        }
    }
}
