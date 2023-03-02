use std::vec;

use fuzzy_matcher::{skim::{SkimMatcherV2, SkimScoreConfig}, FuzzyMatcher, clangd::{self, ClangdMatcher}};
use sqlx::database;
use sublime_fuzzy::{ContinuousMatches, best_match};
use super::*;

pub struct Bridge {
    conn: SqliteConnection,
}
impl Bridge {
    pub async fn new() -> Bridge {
        Bridge{conn: establish_connection().await}
    }
    /* Jots */
    pub async fn update_jot_text(&mut self, id: i64, text: &str, img_path: Option<String>) {
        update_jot(&mut self.conn, id, text, img_path, chrono::Local::now().naive_local()).await.unwrap();
    }
    pub async fn create_jot(&mut self, text: &str, img_path: Option<String>) -> Result<i64, sqlx::Error>{
        let result = insert_jot(&mut self.conn, text, img_path).await?;
        Ok(result.last_insert_rowid())
    }
    pub async fn fetch_new_jot(&mut self, id: i64) -> models::Jot {
        fetch_jot(&mut self.conn, id).await.unwrap().unwrap()
    }
    pub async fn delete_jot(&mut self, id: i64) {
        let tags = self.get_all_tags_for_jot( id).await;
        for tag in tags {
            self.remove_tag_from_jot(tag.id, id).await;
        }
        delete_jot(&mut self.conn, id).await.unwrap();
    }
    /* Tags */
    pub async fn get_all_tags_for_jot(&mut self, id: i64) -> Vec<models::Tag> {
        get_tags_for_jot(&mut self.conn, id).await.unwrap()
    }
    pub async fn get_all_tags(&mut self) -> Vec<models::Tag> {
        get_all_tags(&mut self.conn).await.unwrap()
    }
    pub async fn add_tag_to_jot(&mut self, tag_id: i64, jot_id: i64){
        insert_jot_tag(&mut self.conn, jot_id, tag_id).await.unwrap();
    }
    pub async fn add_new_tag_to_jot(&mut self, title: &str, jot_id: i64) -> models::Tag{
        let tag_id = insert_tag(&mut self.conn, title, Option::None).await.unwrap().last_insert_rowid();
        insert_jot_tag(&mut self.conn, jot_id, tag_id).await.unwrap();
        fetch_tag(&mut self.conn, title).await.unwrap().unwrap()
    }
    pub async fn remove_tag_from_jot(&mut self, tag_id: i64, jot_id: i64){
        delete_jot_tag(&mut self.conn, jot_id, tag_id).await.unwrap();
        if fetch_jot_tag_for_tag(&mut self.conn, tag_id).await.unwrap().is_none() {
            delete_tag(&mut self.conn, tag_id).await.unwrap();
        }
    }
    
    /* Searching */
    pub async fn search_jots(&mut self, query: &str) -> Result<Vec<models::Jot>, sqlx::Error> {
        //TODO: maybe sort by matching ids not the full text
        let jots = get_all_jots(&mut self.conn).await?;
        //fuzzy search tags
        let texts = jots.iter().map(|tag| tag.text.clone()).collect();
        let matching_jot_texts: Vec<String> = fuzzy_search(query, texts).iter()
            .map(|t| t.1.clone()).collect();
        let mut jots: Vec<models::Jot> = jots.iter()
            .filter(|t| {matching_jot_texts.contains(&t.text)})
            .cloned().collect();
        jots.sort_by(|a, b| matching_jot_texts.iter()
            .position(|n| n == &a.text).cmp(&matching_jot_texts.iter().position(|n| n == &b.text)));
        Ok(jots)
    }
    pub async fn smart_search_jots(&mut self, query: &str) -> Result<Vec<models::Jot>, sqlx::Error> {
        let jots = get_all_jots(&mut self.conn).await?;
        let matcher = SkimMatcherV2::default();
        // matcher = matcher.score_config(SkimScoreConfig{gap_start: -6,
        //     gap_extension: -3,
        //     ..Default::default()});
        
        let tags: Vec<String> = get_all_tags(&mut self.conn).await.unwrap().iter().map(|t| t.title.to_owned()).collect();
        let filtered_tags = fuzzy_search(query, tags);
        
        //Score jots
        let mut filtered_jots: Vec<(i64, models::Jot)> = vec![];
        for jot in jots {
            //Fuzzy score
            let mut score = matcher.fuzzy_match(jot.text.as_str(), query).unwrap_or(0);
            
            //Tag score
            let tags = get_tags_for_jot(&mut self.conn, jot.id).await.unwrap();
            let filtered_tag_titles: Vec<String> = filtered_tags.iter().map(|t| t.1.clone()).collect();
            for tag in tags {
                if filtered_tag_titles.contains(&tag.title) {
                    score += 1;
                }
            }

            //Add scored jot to filtered list
            if score > 0 {
                filtered_jots.push((score, jot));
            }
        }
        
        //Sort by score
        filtered_jots.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(filtered_jots.iter().map(|j| j.1.to_owned()).collect())
    }
    
    pub async fn sublime_search_jots(&mut self, query: &str) -> Result<Vec<models::Jot>, sqlx::Error> {
        let jots = get_all_jots(&mut self.conn).await?;
        //let matcher = SkimMatcherV2::default();
        
        let tags: Vec<String> = get_all_tags(&mut self.conn).await.unwrap().iter().map(|t| t.title.to_owned()).collect();
        let filtered_tags = fuzzy_search(query, tags);
        
        //Score jots
        let mut filtered_jots: Vec<(i64, models::Jot)> = vec![];
        for jot in jots {
            //Fuzzy score
            let mut score;
            let result = best_match(query, &jot.text);
            match result {
                Option::Some(s) => score = s.score(),
                Option::None => score = 0
            }
            //let mut score = matcher.fuzzy_match(jot.text.as_str(), query).unwrap_or(0);
            
            //Tag score
            let tags = get_tags_for_jot(&mut self.conn, jot.id).await.unwrap();
            let filtered_tag_titles: Vec<String> = filtered_tags.iter().map(|t| t.1.clone()).collect();
            for tag in tags {
                if filtered_tag_titles.contains(&tag.title) {
                    score += 1;
                }
            }

            //Add scored jot to filtered list
            if score > 0 {
                println!("{}", score);
                filtered_jots.push((score.try_into().unwrap_or(0), jot));
            }
        }
        
        //Sort by score
        filtered_jots.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(filtered_jots.iter().map(|j| j.1.to_owned()).collect())
    }

    pub async fn search_tags(&mut self, query: &str, tag_ids: Vec<i64>) -> Result<Vec<models::Tag>, sqlx::Error>{
        let tags = get_all_tags(&mut self.conn).await?;
        //fuzzy search tags
        let titles = tags.iter().map(|tag| tag.title.clone()).collect();
        let matching_tag_titles: Vec<String> = fuzzy_search(query, titles).iter().map(|t| t.1.clone()).collect();
        //filter out already added tags
        let mut tags: Vec<models::Tag> = tags.iter().filter(|t| !tag_ids.contains(&t.id) && matching_tag_titles.contains(&t.title)).cloned().collect();
        tags.sort_by(|a, b| matching_tag_titles.iter().position(|n| n == &a.title).cmp(&matching_tag_titles.iter().position(|n| n == &b.title)));
        Ok(tags)
    }
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