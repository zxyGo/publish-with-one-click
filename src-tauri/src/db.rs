use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs;
use tauri::{AppHandle, Manager};

pub type DbPool = Pool<Sqlite>;

pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub async fn init(app_handle: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_dir = app_handle.path().app_data_dir()?;
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }
        let db_path = app_dir.join("pwoc.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        if !db_path.exists() {
            fs::File::create(&db_path)?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS article (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                content TEXT NOT NULL,
                markdown TEXT,
                tags TEXT,
                cover TEXT,
                publish_status INTEGER DEFAULT 0,
                publish_time DATETIME,
                publish_platforms TEXT,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await?;

        let columns: Vec<(String,)> = sqlx::query_as("SELECT name FROM pragma_table_info('article')")
            .fetch_all(&pool)
            .await?;
        
        let column_names: Vec<String> = columns.into_iter().map(|(name,)| name).collect();

        let alters = vec![
            ("description", "ALTER TABLE article ADD COLUMN description TEXT"),
            ("markdown", "ALTER TABLE article ADD COLUMN markdown TEXT"),
            ("tags", "ALTER TABLE article ADD COLUMN tags TEXT"),
            ("cover", "ALTER TABLE article ADD COLUMN cover TEXT"),
            ("publish_status", "ALTER TABLE article ADD COLUMN publish_status INTEGER DEFAULT 0"),
            ("publish_time", "ALTER TABLE article ADD COLUMN publish_time DATETIME"),
            ("publish_platforms", "ALTER TABLE article ADD COLUMN publish_platforms TEXT"),
            ("updated_at", "ALTER TABLE article ADD COLUMN updated_at DATETIME DEFAULT CURRENT_TIMESTAMP"),
            ("created_at", "ALTER TABLE article ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP"),
        ];

        for (col, sql) in alters {
            if !column_names.contains(&col.to_string()) {
                let _ = sqlx::query(sql).execute(&pool).await;
            }
        }

        Ok(Database { pool })
    }
}
