use tauri::api::path::download_dir;

use crate::dao::imageview_dao::{BrowseSettings, ImageViewDao, ImagesMeta, ImagesMetaList};
use crate::model::*;

pub fn get_dao() -> ImageViewDao {
    // let dir = std::env::current_dir().unwrap().join("data");
    let dir = dire::Directories::with_prefix("imageview", "ImageView").unwrap();
    let path = dir.data_home();
    let create_result = std::fs::create_dir_all(&path);
    match create_result {
        Ok(_) => (),
        // Err(error) => println!("创建data目录失败 {:?}", error),
        Err(_) => (),
    }
    ImageViewDao::new(path.join("images.db"))
}

#[tauri::command]
pub fn init_table(_window: tauri::Window) -> Result<(), String> {
    let dao = get_dao();
    dao.init_table();
    Ok(())
}

#[tauri::command]
pub fn add_images_meta(
    _window: tauri::Window,
    path: &str,
    title: &str,
    author: &str,
    intro: &str,
    cover: &str,
) -> Result<(), String> {
    let dao = get_dao();
    dao.add_images_meta(path, title, author, intro, cover);
    Ok(())
}

#[tauri::command]
pub fn get_images_meta_list(
    _window: tauri::Window,
    search: &str,
    page: i64,
    page_size: i64,
) -> Result<ImagesMetaList, String> {
    println!("获取 image meta");
    let dao = get_dao();
    dao.get_images_meta_list(search, page, page_size)
}

pub fn is_image_by_suffix(s: &String) -> bool {
    let s = s.to_lowercase();
    for suffix in [".jpeg", ".jpg", ".webp", ".gif", ".png", ".bmp"] {
        if s.ends_with(suffix) {
            return true;
        }
    }
    false
}

#[tauri::command]
pub fn get_images_folder_info(
    _window: tauri::Window,
    path_str: &str,
) -> Result<ImagesFolderInfo, String> {
    // 前端传个文件夹路径过来，后台返回目录相关信息
    let path = std::path::PathBuf::from(path_str);
    let name = String::from(path.file_name().unwrap().to_str().unwrap());

    // 读二级目录里面的图片文件
    let mut info = ImagesFolderInfo {
        name: name,
        files: vec![],
    };
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let mut files2: Vec<String> = vec![];
            if entry.path().is_dir() {
                for entry2 in entry.path().read_dir().expect("read_dir call failed") {
                    if let Ok(entry2) = entry2 {
                        let fp = String::from(entry2.path().to_str().unwrap());
                        if is_image_by_suffix(&fp) {
                            files2.push(fp);
                        }
                    }
                }
                info.files.push(ImagesFolderInfoFile {
                    is_dir: entry.path().is_dir(),
                    path: String::from(entry.path().to_str().unwrap()),
                    name: String::from(entry.path().file_name().unwrap().to_str().unwrap()),
                    files: files2,
                });
            } else {
                let fp = String::from(entry.path().to_str().unwrap());
                if is_image_by_suffix(&fp) {
                    info.files.push(ImagesFolderInfoFile {
                        is_dir: entry.path().is_dir(),
                        path: fp,
                        name: String::from(entry.path().file_name().unwrap().to_str().unwrap()),
                        files: files2,
                    });
                }
            }
        }
    }

    Ok(info)
}

#[tauri::command]
pub fn get_images_meta(_window: tauri::Window, id: i64) -> Result<ImagesMeta, String> {
    let dao = get_dao();
    dao.get_images_meta(id)
}

#[tauri::command]
pub fn update_browse_settings(
    _window: tauri::Window,
    meta_id: i64,
    browse_type: &str,
    home_page: bool,
    current_path: &str,
    current_index: i64,
) -> Result<(), String> {
    let dao = get_dao();
    dao.update_browse_settings(meta_id, browse_type, home_page, current_path, current_index);
    Ok(())
}

#[tauri::command]
pub fn get_browse_settings(_window: tauri::Window, meta_id: i64) -> Result<BrowseSettings, String> {
    let dao = get_dao();
    dao.get_browse_settings(meta_id)
}

#[tauri::command]
pub fn delete_images_meta(_window: tauri::Window, id: i64) -> Result<(), String> {
    let dao = get_dao();
    dao.delete_images_meta(id)
}

#[tauri::command]
pub fn update_images_meta(
    _window: tauri::Window,
    id: i64,
    path: &str,
    title: &str,
    author: &str,
    intro: &str,
    cover: &str,
) -> Result<(), String> {
    let dao = get_dao();
    dao.update_images_meta(id, path, title, author, intro, cover);
    Ok(())
}

#[tauri::command]
pub fn hide_window(_window: tauri::Window) -> Result<(), String> {
    _window.hide().expect("关闭失败");
    Ok(())
}

#[tauri::command]
pub fn upload_file(
    _window: tauri::Window,
    bytes: Vec<u8>,
    file_name: &str,
    file_type: &str,
) -> Result<(), String> {
    use std::io::Write;
    use uuid::Uuid;
    use crate::util::Path;

    let base_path = Path::base_path();
    let file_path = base_path.join(format!("{}-{}", Uuid::new_v4().to_string(), file_name));

    let mut file = std::fs::File::create(file_path.as_os_str()).expect("文件上传失败");
    file.write_all(&bytes).expect("文件写入失败");

    println!(
        "data written to file success! 文件的地址为 {:?}, 文件的大小为: {}",
        file_path.as_os_str(),
        bytes.len()
    );

    get_dao().add_file_meta(
        file_path.as_os_str().to_str().unwrap(),
        file_name, 
        file_type, 
        bytes.len() as i64
    );

    Ok(())
}

#[tauri::command]
pub fn get_file_meta_list(
    _window: tauri::Window,
    search: &str,
    page: i64,
    page_size: i64,
) -> Result<FileMetaList, String> {
    let dao = get_dao();
    dao.get_file_meta_list(search, page, page_size)
}