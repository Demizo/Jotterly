#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::Manager;
extern crate tauri_test;
use tauri_test::{database::{self}};
use database::bridge::Bridge;
pub mod theme;
pub mod settings;

#[tauri::command]
async fn search_jots(query: String, active_tags: Vec<i64>) -> Vec<database::models::Jot> {
    let mut bridge = Bridge::new().await;
    if query.trim().len() == 0 && active_tags.len() == 0 {
        bridge.get_recent_jots(5).await
    } else {
        bridge.sublime_search_jots(query.as_str(), active_tags).await.unwrap()
    }
}
//TODO: this method is not being used
#[tauri::command]
async fn get_jot(id: i64) -> database::models::Jot {
    let mut bridge = Bridge::new().await;
    bridge.fetch_new_jot(id).await
}
#[tauri::command]
async fn search_tags(query: String, tag_ids: Vec<i64>) -> Vec<database::models::Tag> {
    let mut bridge = Bridge::new().await;
    bridge.search_tags(query.as_str(), tag_ids).await.unwrap()
}
#[tauri::command]
async fn get_all_tags_for_jot(id: i64) -> Vec<database::models::Tag> {
    let mut bridge = Bridge::new().await;
    bridge.get_all_tags_for_jot(id).await
}
#[tauri::command]
async fn get_all_tags() -> Vec<database::models::Tag> {
    let mut bridge = Bridge::new().await;
    bridge.get_all_tags().await
}
#[tauri::command]
async fn get_top_tags() -> Vec<database::models::Tag> {
    let mut bridge = Bridge::new().await;
    bridge.get_top_tags().await
}
#[tauri::command]
async fn add_tag_to_jot(tag_id: i64, jot_id: i64){
    let mut bridge = Bridge::new().await;
    bridge.add_tag_to_jot(tag_id, jot_id).await;
}
#[tauri::command]
async fn add_new_tag_to_jot(title: String, jot_id: i64) -> database::models::Tag{
    let mut bridge = Bridge::new().await;
    bridge.add_new_tag_to_jot(title.as_str(), jot_id).await
}
#[tauri::command]
async fn remove_tag_from_jot(tag_id: i64, jot_id: i64){
    let mut bridge = Bridge::new().await;
    bridge.remove_tag_from_jot(tag_id, jot_id).await;
}
#[tauri::command]
async fn update_jot_text(id: i64, text: String, img_path: Option<String>) {
    let mut bridge = Bridge::new().await;
    bridge.update_jot_text(id, text.as_str(), img_path).await;
}
#[tauri::command]
async fn create_jot(text: String, img_path: Option<String>, tag_ids: Vec<i64>) -> i64 {
    let mut bridge = Bridge::new().await;
    bridge.create_jot(text.as_str(), img_path, tag_ids).await.unwrap()
}
#[tauri::command]
async fn delete_jot(id: i64) {
    let mut bridge = Bridge::new().await;
    bridge.delete_jot(id).await;
}
//Themes
#[tauri::command]
async fn fetch_all_themes() -> Vec<String>{
    theme::fetch_all_themes()
}
#[tauri::command]
async fn get_theme(theme: String) -> String{
    theme::get_theme(theme)
}
//Settings
#[tauri::command]
async fn get_settings() -> String{
    settings::get_settings()
}
#[tauri::command]
async fn set_setting(key: String, val: String){
    settings::set_setting(key, val).unwrap();
}
fn init(app: &mut tauri::App) {
    settings::create_default_settings(app).unwrap();
    theme::create_default_themes(app).unwrap();
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
        let main_window = app.get_window("main").unwrap();
        
        init(app);
        
        tauri::async_runtime::spawn(async move {
            main_window.show().unwrap();
          });
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_jots,get_all_tags_for_jot,get_all_tags,get_top_tags,add_tag_to_jot,add_new_tag_to_jot,remove_tag_from_jot,search_tags,update_jot_text,create_jot,delete_jot,get_jot,
            fetch_all_themes,get_theme,
            get_settings,set_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}