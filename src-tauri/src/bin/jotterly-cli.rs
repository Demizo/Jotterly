use fuzzy_matcher::{skim::{SkimMatcherV2}, FuzzyMatcher};
use sqlx::{SqliteConnection, sqlite::SqliteQueryResult};
use std::{io::{stdin, Read}};
extern crate tauri_test;
use tauri_test::{database};
#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = database::establish_connection().await;
    
    loop {
        //Prompt s = search, c=create
        let answer = main_prompt();
        if answer.eq("n") {
            //create new jot
            new_jot(&mut conn).await?;
        } else if answer.eq("q") {
            return Ok(());
        } else if answer.eq("s"){
            search(&mut conn).await?;
        }
    }
    //Search Jots
    //list jots
    //select to open
    //close or edit (or tag)?

    //edit/list tags
    
}
fn main_prompt() -> String {
    println!("Welcome to Jotterly:\ns: search\nn: new jot\nq: quit");
    get_response()
}
async fn search(conn: &mut SqliteConnection) -> Result<(), sqlx::Error>{
    
    println!("Search...");
    let query = get_response();
    let jot_results = search_jots(conn, query.as_str()).await?;
    //let tag_results = search_tags(conn, query.as_str(), tag_ids);
    for result in jot_results {
        let tags = database::get_tags_for_jot(conn, result.id).await?;
        println!("{}: {}", result.id, result.text);
        let mut tag_list = String::new();
        for tag in tags {
            tag_list += format!("#{} ", tag.title).as_str();
        }
        println!("{}\n--------------", tag_list);
    }
    Ok(())
}
async fn new_jot(conn: &mut SqliteConnection) -> Result<SqliteQueryResult, sqlx::Error>{ 
    let mut body = String::new();
    let mut tag_ids: Vec<i64> = Vec::new();
    #[cfg(not(windows))]
    const EOF: &str = "CTRL+D";
    
    #[cfg(windows)]
    const EOF: &str = "CTRL+Z";
    println!(
        "\nJot it down! (Press {} when finished)\n",
        EOF
    );
    stdin().read_to_string(&mut body).unwrap();
    
    loop {
        println!("Add Tags?\ny: yes\nn: no");
        let answer = get_response();
        if answer.eq("y") {
            //search for a tag
            println!("Search...");
            let query = get_response();
            let tags = search_tags(conn, query.as_str(), &tag_ids).await?;
            //select or create tag
            println!("Select a tag or create a new one...\nb: back");
            //prompt new tag
            if database::fetch_tag(conn, &query).await?.is_none() && !query.is_empty() {
                println!("n: create tag: {}", query);
            }
            for tag in tags {
                println!("{}: add #{}", tag.id, tag.title);
            }
            let answer = get_response();
            if answer.eq("n") {
                //Create a new tag and add it
                let id = database::insert_tag(conn, query.as_str(), Option::None).await?;
                tag_ids.push(id.last_insert_rowid());
            } else if answer.eq("b") {
                //go back
                continue;
            } else {
                //Add existing tag
                let answer: i64 = answer.parse().unwrap();
                //TODO: this could be any number including a tag already added
                tag_ids.push(answer);
            }
        } else {
            break;
        }
    }
    
    tauri_test::create_jot(conn, &body, Option::None, Some(tag_ids)).await
}

async fn search_jots(conn: &mut SqliteConnection, query: &str) -> Result<Vec<database::models::Jot>, sqlx::Error> {
    //TODO: maybe sort by matching ids not the full text
    let jots = database::get_all_jots(conn).await?;
    //fuzzy search tags
    let texts = jots.iter().map(|tag| tag.text.clone()).collect();
    let matching_jot_texts: Vec<String> = fuzzy_search(query, texts).iter().map(|t| t.1.clone()).collect();
    //filter out already added tags
    let mut jots: Vec<database::models::Jot> = jots.iter().filter(|t| matching_jot_texts.contains(&t.text)).cloned().collect();
    jots.sort_by(|a, b| matching_jot_texts.iter().position(|n| n == &a.text).cmp(&matching_jot_texts.iter().position(|n| n == &b.text)));
    Ok(jots)
}

async fn search_tags(conn: &mut SqliteConnection, query: &str, tag_ids: &Vec<i64>) -> Result<Vec<database::models::Tag>, sqlx::Error>{
    let tags = database::get_all_tags(conn).await?;
    //fuzzy search tags
    let titles = tags.iter().map(|tag| tag.title.clone()).collect();
    let matching_tag_titles: Vec<String> = fuzzy_search(query, titles).iter().map(|t| t.1.clone()).collect();
    //filter out already added tags
    let mut tags: Vec<database::models::Tag> = tags.iter().filter(|t| !tag_ids.contains(&t.id) && matching_tag_titles.contains(&t.title)).cloned().collect();
    tags.sort_by(|a, b| matching_tag_titles.iter().position(|n| n == &a.title).cmp(&matching_tag_titles.iter().position(|n| n == &b.title)));
    Ok(tags)
}
fn get_response() -> String {
    let mut answer = String::new();
    stdin().read_line(&mut answer).unwrap();
    answer.trim_end().to_string()
}
fn fuzzy_search(query: &str, list: Vec<String>) -> Vec<(i64, String)> {
    let matcher = SkimMatcherV2::default();
    let mut results = vec![];
        
    for item in list {
        let score = matcher.fuzzy_match(item.as_str(), query);
        if score.is_some() {
            results.push((score.unwrap(), item));
        }
    }
    
    
    results.sort_by(|a, b| b.0.cmp(&a.0));
    results.iter().map(|(score, value)| {
        (score.to_owned(), value.clone().to_owned().to_string())
    }).collect()
}