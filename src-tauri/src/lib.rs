use database::{fetch_tag};
use sqlx::{SqliteConnection, sqlite::SqliteQueryResult};

pub mod database;

pub struct Jot {
    pub id: i64,
    pub text: String,
    pub img_path: Option<String>
}
pub struct Tag {
    pub title: String,
    pub color: Option<String>,
    pub priority: i64
}
pub async fn create_jot(conn: &mut SqliteConnection, text: &str, img_path: Option<&str>, tags: Option<Vec<i64>>) -> Result<SqliteQueryResult, sqlx::Error>{
    let jot_result = database::insert_jot(conn, text, img_path).await?;
    //if no tags skip
    if tags.is_none() { 
        return Ok(jot_result); 
    }
    for id in tags.unwrap() {
        database::insert_jot_tag(conn, jot_result.last_insert_rowid(), id).await?;
    }
    Ok(jot_result)
}
pub async fn update_tag(conn: &mut SqliteConnection, title: &str, color: Option<&str>, priority: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    let db_tag = fetch_tag(conn, title).await?;
    Ok(database::update_tag(conn, db_tag.unwrap().id, title, color, priority, chrono::offset::Local::now().naive_local()).await?)
}
/*
//TODO: fetch_or_create? create_or_update?
pub async fn fetch_or_create(conn: &mut SqliteConnection, title: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut db_tag: Option<models::Tag> = fetch_tag(conn, title).await.unwrap();
    if db_tag.is_none() {
        database::insert_tag(conn, title, Option::None, Option::None).await?;
        db_tag = fetch_tag(conn, title).await?;
    }
}*/
//TODO: start handling errors... no crashing ;(