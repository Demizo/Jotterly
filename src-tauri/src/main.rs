#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate tauri_test;
use tauri_test::{database};
use database::bridge::Bridge;

#[tauri::command]
async fn search_jots(query: String) -> Vec<database::models::Jot> {
    let mut bridge = Bridge::new().await;
    bridge.sublime_search_jots(query.as_str()).await.unwrap()
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
async fn create_jot(text: String, img_path: Option<String>) -> i64 {
    let mut bridge = Bridge::new().await;
    bridge.create_jot(text.as_str(), img_path).await.unwrap()
}
#[tauri::command]
async fn delete_jot(id: i64) {
    let mut bridge = Bridge::new().await;
    bridge.delete_jot(id).await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_jots,get_all_tags_for_jot,get_all_tags,add_tag_to_jot,add_new_tag_to_jot,remove_tag_from_jot,search_tags,update_jot_text,create_jot,delete_jot,get_jot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}