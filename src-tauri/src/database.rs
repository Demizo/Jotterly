use std::{env};

use dotenvy::dotenv;
use sqlx::{SqliteConnection, Connection, sqlite::SqliteQueryResult};

pub mod models;
pub mod bridge;
pub async fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::connect(&database_url).await.expect("Can't connect to {database_url}")
}

/* Insert Entries */
pub async fn insert_jot(conn: &mut SqliteConnection,
    text: &str,
    img_path: Option<&str>
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        INSERT INTO jots (text, img_path)
        VALUES (?,?)
        ",
        text,
        img_path
    ).execute(conn).await;
    result
}
pub async fn insert_tag(conn: &mut SqliteConnection,
    title: &str,
    color: Option<String>
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        INSERT INTO tags (title, color)
        VALUES (?,?)
        ",
        title,
        color
    ).execute(conn).await;
    result
}
pub async fn insert_jot_tag(conn: &mut SqliteConnection,
    jot_id: i64,
    tag_id: i64
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        INSERT INTO jot_tags (jot_id, tag_id)
        VALUES (?,?)
        ",
        jot_id,
        tag_id,
    ).execute(conn).await;
    result
}

/* Update Entries */
pub async fn update_jot(conn: &mut SqliteConnection,
    id: i64,
    text: &str,
    img_path: Option<&str>,
    time_modified: chrono::NaiveDateTime
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        UPDATE jots SET
        text = ?,
        img_path = ?,
        time_modified = ?
        WHERE jots.id = ?
        ",
        text,
        img_path,
        time_modified,
        id
    ).execute(conn).await;
    result
}
pub async fn update_tag(conn: &mut SqliteConnection,
    id: i64,
    title: &str,
    color: Option<&str>,
    priority: i64,
    time_modified: chrono::NaiveDateTime
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        UPDATE tags SET
        title = ?,
        color = ?,
        priority = ?,
        time_modified = ?
        WHERE tags.id = ?
        ",
        title,
        color,
        priority,
        time_modified,
        id
    ).execute(conn).await;
    result
}

/* Delete Entries */
pub async fn delete_jot(conn: &mut SqliteConnection,
    id: i64
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        DELETE FROM jots
        WHERE jots.id = ?
        ",
        id
    ).execute(conn).await;
    result
}
pub async fn delete_tag(conn: &mut SqliteConnection,
    id: i64
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        DELETE FROM tags
        WHERE tags.id = ?
        ",
        id
    ).execute(conn).await;
    result
}
pub async fn delete_jot_tag(conn: &mut SqliteConnection,
    jot_id: i64,
    tag_id: i64
    ) -> Result<SqliteQueryResult, sqlx::Error>{
    let result = sqlx::query!(
        "
        DELETE FROM jot_tags
        WHERE jot_id = ? AND tag_id = ?
        ",
        jot_id,
        tag_id
    ).execute(conn).await;
    result
}

/* Get Data */
pub async fn get_all_jots(conn: &mut SqliteConnection) -> Result<Vec<models::Jot>, sqlx::Error> {
    let jots = sqlx::query_as!(models::Jot,
        "
        SELECT id, text, img_path, time_create, time_modified
        FROM jots;
        "
    ).fetch_all(conn).await;
    jots
}
pub async fn get_all_tags(conn: &mut SqliteConnection) -> Result<Vec<models::Tag>, sqlx::Error> {
    let tags = sqlx::query_as!(models::Tag,
        "
        SELECT id, title, color, priority, time_create, time_modified
        FROM tags;
        "
    ).fetch_all(conn).await;
    tags
}
pub async fn get_tags_for_jot(conn: &mut SqliteConnection, id: i64) -> Result<Vec<models::Tag>, sqlx::Error> {
    let tags = sqlx::query_as!(models::Tag,
        "
        SELECT t.id, t.title, t.color, t.priority, t.time_create, t.time_modified
        FROM jot_tags jt
        INNER JOIN tags t ON t.id = jt.tag_id
        WHERE jt.jot_id = ?
        ",
        id
    ).fetch_all(conn).await;
    tags
}
pub async fn get_jots_for_tag(conn: &mut SqliteConnection, id: i64) -> Result<Vec<models::Jot>, sqlx::Error> {
    let jots = sqlx::query_as!(models::Jot,
        "
        SELECT j.id, j.text, j.img_path, j.time_create, j.time_modified
        FROM jot_tags jt
        INNER JOIN jots j ON j.id = jt.tag_id
        WHERE jt.jot_id = ?
        ",
        id
    ).fetch_all(conn).await;
    jots
}

/* Search Data */
pub async fn fetch_tag(conn: &mut SqliteConnection, title: &str) -> Result<Option<models::Tag>, sqlx::Error> {
    let tag = sqlx::query_as!(models::Tag,
        "
        SELECT * FROM tags WHERE title = ?
        ",
        title
    ).fetch_optional(conn).await;
    tag
}
pub async fn fetch_jot_tag_for_tag(conn: &mut SqliteConnection, tag_id: i64) -> Result<Option<models::JotTag>, sqlx::Error> {
    let jot_tag = sqlx::query_as!(models::JotTag,
        "
        SELECT * FROM jot_tags WHERE tag_id = ?
        ",
        tag_id
    ).fetch_optional(conn).await;
    jot_tag
}