use tauri::State;
use crate::db::Database;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub markdown: Option<String>,
    pub tags: Option<String>,
    pub cover: Option<String>,
    pub publish_status: Option<i64>,
    pub publish_time: Option<chrono::NaiveDateTime>,
    pub publish_platforms: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub desc: Option<String>,
    pub cover: Option<String>,
    pub content: String,
    pub markdown: Option<String>,
}

#[tauri::command]
pub async fn get_article_list(state: State<'_, Database>) -> Result<Vec<Article>, String> {
    let articles = sqlx::query_as::<_, Article>("SELECT * FROM article ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(articles)
}

#[tauri::command]
pub async fn create_article(
    state: State<'_, Database>,
    article: CreateArticleRequest,
) -> Result<i64, String> {
    let id = sqlx::query(
        "INSERT INTO article (title, description, cover, content, markdown) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&article.title)
    .bind(&article.desc)
    .bind(&article.cover)
    .bind(&article.content)
    .bind(&article.markdown)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?
    .last_insert_rowid();

    Ok(id)
}
