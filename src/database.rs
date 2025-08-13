use sqlx::{SqlitePool, Row};
use anyhow::Result;
use tracing::{info, error};

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing database schema");
        
        // Read and execute the schema
        let schema = include_str!("../database_schema.sql");
        
        // Split the schema into individual statements and execute them
        for statement in schema.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                sqlx::query(statement)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| {
                        error!("Failed to execute schema statement: {}", e);
                        e
                    })?;
            }
        }
        
        info!("Database schema initialized successfully");
        Ok(())
    }

    pub async fn get_ingestion_logs(&self, limit: Option<i32>) -> Result<Vec<IngestionLog>> {
        let limit = limit.unwrap_or(10);
        let rows = sqlx::query(
            r#"
            SELECT 
                id,
                source,
                file_name,
                trade_date,
                records_processed,
                status,
                error_message,
                started_at,
                completed_at
            FROM ingestion_log 
            ORDER BY completed_at DESC 
            LIMIT ?
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(IngestionLog {
                id: row.get("id"),
                source: row.get("source"),
                file_name: row.get("file_name"),
                trade_date: row.get("trade_date"),
                records_processed: row.get("records_processed"),
                status: row.get("status"),
                error_message: row.get("error_message"),
                started_at: row.get("started_at"),
                completed_at: row.get("completed_at"),
            });
        }

        Ok(logs)
    }

    pub async fn log_ingestion(&self, log: &IngestionLogInsert) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ingestion_log (
                source, file_name, trade_date, records_processed, 
                status, error_message, started_at, completed_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&log.source)
        .bind(&log.file_name)
        .bind(&log.trade_date)
        .bind(&log.records_processed)
        .bind(&log.status)
        .bind(&log.error_message)
        .bind(&log.started_at)
        .bind(&log.completed_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct IngestionLog {
    pub id: i64,
    pub source: String,
    pub file_name: Option<String>,
    pub trade_date: Option<chrono::NaiveDate>,
    pub records_processed: Option<i64>,
    pub status: String,
    pub error_message: Option<String>,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub completed_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct IngestionLogInsert {
    pub source: String,
    pub file_name: Option<String>,
    pub trade_date: Option<chrono::NaiveDate>,
    pub records_processed: Option<i64>,
    pub status: String,
    pub error_message: Option<String>,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub completed_at: chrono::NaiveDateTime,
}
