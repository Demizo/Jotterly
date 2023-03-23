use std::{fs::{self, File}, path::PathBuf};

use serde_json::Value;

pub fn set_setting(key: String, val: String) -> std::io::Result<()> {
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    path.push("settings.json");
    let file = File::open(&path)?;
    let mut settings: Value = serde_json::from_reader(file)?;
    
    settings[key] = val.into();
    
    let file = File::create(&path)?;
    serde_json::to_writer(file, &settings)?;
    
    Ok(())
}
pub fn get_settings() -> String {
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    path.push("settings.json");
    String::from_utf8(fs::read(path).unwrap()).unwrap()
}
pub fn create_default_settings(app: &mut tauri::App) -> std::io::Result<()> {
    let mut path = dirs::config_dir().expect("could not get config dir");
    path.push("jotterly");
    std::fs::create_dir_all(&path).expect("could not create dir");
    path.push("settings.json");

    let new_settings = !PathBuf::from(&path).exists();
    if new_settings {
        let resource_path = app.path_resolver()
            .resolve_resource("resources/settings.json")
            .expect("failed to resolve resource");
        let file_bytes = fs::read(resource_path).unwrap();
        
        fs::write(&path, file_bytes).expect("Failed to write file");
    }
    Ok(())
}