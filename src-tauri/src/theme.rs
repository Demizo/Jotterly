use std::{fs::{self}};

pub fn fetch_all_themes() -> Vec<String> {
    let mut themes_list = vec![];
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    path.push("themes");
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let entry_name = entry.file_name().to_str().unwrap().trim_end_matches(".json").to_string();
                    themes_list.push(entry_name);
                }                
            }
        }
    }
    themes_list
}
pub fn get_theme(theme: String) -> String {
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    path.push("themes");
    path.push(theme + ".json");
    String::from_utf8(fs::read(path).unwrap()).unwrap()
}
pub fn create_default_themes(app: &mut tauri::App) -> std::io::Result<()> {
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    std::fs::create_dir_all(&path).expect("could not create dir");
    path.push("themes");
    std::fs::create_dir_all(&path).expect("could not create dir");
    
    let resource_path = app.path_resolver()
        .resolve_resource("resources/themes/")
        .expect("failed to resolve resource");
    
    if let Ok(entries) = fs::read_dir(resource_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let file_bytes = fs::read(entry.path()).unwrap();
                    path.push(entry.file_name());
                    fs::write(&path, file_bytes).expect("Failed to write file");
                    path.pop();
                }                
            }
        }
    }
    Ok(())
}