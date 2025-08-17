use sqlx::{SqlitePool, Row};
use anyhow::Result;
use tracing::{info, error};

#[derive(Clone)]
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

    pub async fn upsert_company(&self, symbol: &str, isin: &str, series: &str) -> Result<i64> {
        // Try to get existing company
        let existing = sqlx::query(
            "SELECT id FROM companies WHERE symbol = ?"
        )
        .bind(symbol)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing {
            let id: i64 = row.get("id");
            // Update the existing record
            sqlx::query(
                "UPDATE companies SET isin = ?, series = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
            )
            .bind(isin)
            .bind(series)
            .bind(id)
            .execute(&self.pool)
            .await?;
            
            Ok(id)
        } else {
            // Insert new company
            let result = sqlx::query(
                "INSERT INTO companies (symbol, isin, series) VALUES (?, ?, ?)"
            )
            .bind(symbol)
            .bind(isin)
            .bind(series)
            .execute(&self.pool)
            .await?;
            
            Ok(result.last_insert_rowid())
        }
    }

    pub async fn upsert_daily_price(&self, company_id: i64, price_data: &crate::nse::StockRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO daily_prices (
                company_id, trade_date, open_price, high_price, low_price, 
                close_price, last_price, prev_close, total_traded_qty, 
                total_traded_value, total_trades
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(company_id)
        .bind(price_data.timestamp)
        .bind(price_data.open)
        .bind(price_data.high)
        .bind(price_data.low)
        .bind(price_data.close)
        .bind(price_data.last)
        .bind(price_data.prevclose)
        .bind(price_data.tottrdqty)
        .bind(price_data.tottrdval)
        .bind(price_data.totaltrades)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_stock_records(&self, records: &[crate::nse::StockRecord]) -> Result<usize> {
        let mut stored_count = 0;
        
        for record in records {
            // First ensure the company exists
            let company_id = self.upsert_company(&record.symbol, &record.isin, &record.series).await?;
            
            // Then store the price data
            self.upsert_daily_price(company_id, record).await?;
            stored_count += 1;
        }
        
        info!("Stored {} stock records in database", stored_count);
        Ok(stored_count)
    }

    // API-specific query methods
    pub async fn get_companies(&self, limit: u32, offset: u32, search: Option<&str>, series: Option<&str>) -> Result<(Vec<Company>, u64)> {
        // Build the query dynamically based on filters
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, symbol, isin, series, name, created_at, updated_at FROM companies"
        );
        
        let mut conditions = Vec::new();
        
        if let Some(search_term) = search {
            conditions.push(format!("(symbol LIKE '%{}%' OR name LIKE '%{}%')", search_term, search_term));
        }
        
        if let Some(series_filter) = series {
            conditions.push(format!("series = '{}'", series_filter));
        }
        
        if !conditions.is_empty() {
            query_builder.push(" WHERE ");
            query_builder.push(conditions.join(" AND "));
        }
        
        query_builder.push(" ORDER BY symbol LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);
        
        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;
        
        let mut companies = Vec::new();
        for row in rows {
            companies.push(Company {
                id: row.get("id"),
                symbol: row.get("symbol"),
                isin: row.get("isin"),
                series: row.get("series"),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        // Get total count
        let mut count_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) as count FROM companies");
        if !conditions.is_empty() {
            count_builder.push(" WHERE ");
            count_builder.push(conditions.join(" AND "));
        }
        
        let count_row = count_builder.build().fetch_one(&self.pool).await?;
        let total: u64 = count_row.get::<i64, _>("count") as u64;
        
        Ok((companies, total))
    }
    
    pub async fn get_company_prices(&self, symbol: &str, limit: u32, offset: u32, from_date: Option<&str>, to_date: Option<&str>) -> Result<(Vec<StockPrice>, u64)> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"SELECT dp.id, dp.company_id, c.symbol, dp.trade_date, dp.open_price, dp.high_price, 
               dp.low_price, dp.close_price, dp.last_price, dp.prev_close, dp.total_traded_qty, 
               dp.total_traded_value, dp.total_trades, dp.created_at
               FROM daily_prices dp
               JOIN companies c ON dp.company_id = c.id
               WHERE c.symbol = "#
        );
        query_builder.push_bind(symbol);
        
        if let Some(from) = from_date {
            query_builder.push(" AND dp.trade_date >= ");
            query_builder.push_bind(from);
        }
        
        if let Some(to) = to_date {
            query_builder.push(" AND dp.trade_date <= ");
            query_builder.push_bind(to);
        }
        
        query_builder.push(" ORDER BY dp.trade_date DESC LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);
        
        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;
        
        let mut prices = Vec::new();
        for row in rows {
            prices.push(StockPrice {
                id: row.get("id"),
                company_id: row.get("company_id"),
                symbol: row.get("symbol"),
                trade_date: row.get("trade_date"),
                open_price: row.get("open_price"),
                high_price: row.get("high_price"),
                low_price: row.get("low_price"),
                close_price: row.get("close_price"),
                last_price: row.get("last_price"),
                prev_close: row.get("prev_close"),
                total_traded_qty: row.get("total_traded_qty"),
                total_traded_value: row.get("total_traded_value"),
                total_trades: row.get("total_trades"),
                created_at: row.get("created_at"),
            });
        }
        
        // Get count for pagination
        let mut count_builder = sqlx::QueryBuilder::new(
            "SELECT COUNT(*) as count FROM daily_prices dp JOIN companies c ON dp.company_id = c.id WHERE c.symbol = "
        );
        count_builder.push_bind(symbol);
        
        if let Some(from) = from_date {
            count_builder.push(" AND dp.trade_date >= ");
            count_builder.push_bind(from);
        }
        
        if let Some(to) = to_date {
            count_builder.push(" AND dp.trade_date <= ");
            count_builder.push_bind(to);
        }
        
        let count_row = count_builder.build().fetch_one(&self.pool).await?;
        let total: u64 = count_row.get::<i64, _>("count") as u64;
        
        Ok((prices, total))
    }
    
    pub async fn get_latest_prices(&self, limit: u32, offset: u32, date: Option<&str>, series: Option<&str>) -> Result<(Vec<StockPrice>, u64)> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"SELECT dp.id, dp.company_id, c.symbol, dp.trade_date, dp.open_price, dp.high_price,
               dp.low_price, dp.close_price, dp.last_price, dp.prev_close, dp.total_traded_qty,
               dp.total_traded_value, dp.total_trades, dp.created_at
               FROM daily_prices dp
               JOIN companies c ON dp.company_id = c.id"#
        );
        
        let mut conditions = Vec::new();
        
        if let Some(trade_date) = date {
            conditions.push(format!("dp.trade_date = '{}'", trade_date));
        } else {
            // Get the latest trading date
            let latest_date_query = "SELECT MAX(trade_date) as max_date FROM daily_prices";
            let latest_row = sqlx::query(latest_date_query).fetch_one(&self.pool).await?;
            let latest_date: Option<chrono::NaiveDate> = latest_row.get("max_date");
            
            if let Some(date) = latest_date {
                conditions.push(format!("dp.trade_date = '{}'", date));
            }
        }
        
        if let Some(series_filter) = series {
            conditions.push(format!("c.series = '{}'", series_filter));
        }
        
        if !conditions.is_empty() {
            query_builder.push(" WHERE ");
            query_builder.push(conditions.join(" AND "));
        }
        
        query_builder.push(" ORDER BY dp.total_traded_value DESC LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);
        
        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;
        
        let mut prices = Vec::new();
        for row in rows {
            prices.push(StockPrice {
                id: row.get("id"),
                company_id: row.get("company_id"),
                symbol: row.get("symbol"),
                trade_date: row.get("trade_date"),
                open_price: row.get("open_price"),
                high_price: row.get("high_price"),
                low_price: row.get("low_price"),
                close_price: row.get("close_price"),
                last_price: row.get("last_price"),
                prev_close: row.get("prev_close"),
                total_traded_qty: row.get("total_traded_qty"),
                total_traded_value: row.get("total_traded_value"),
                total_trades: row.get("total_trades"),
                created_at: row.get("created_at"),
            });
        }
        
        // Get count for pagination
        let mut count_builder = sqlx::QueryBuilder::new(
            "SELECT COUNT(*) as count FROM daily_prices dp JOIN companies c ON dp.company_id = c.id"
        );
        
        if !conditions.is_empty() {
            count_builder.push(" WHERE ");
            count_builder.push(conditions.join(" AND "));
        }
        
        let count_row = count_builder.build().fetch_one(&self.pool).await?;
        let total: u64 = count_row.get::<i64, _>("count") as u64;
        
        Ok((prices, total))
    }
    
    pub async fn search_companies(&self, query: &str, limit: u32, offset: u32) -> Result<(Vec<CompanyWithLatestPrice>, u64)> {
        let search_query = r#"
            SELECT DISTINCT c.id, c.symbol, c.isin, c.series, c.name,
                   dp.close_price as latest_price, dp.trade_date as latest_date
            FROM companies c
            LEFT JOIN daily_prices dp ON c.id = dp.company_id
            AND dp.trade_date = (SELECT MAX(trade_date) FROM daily_prices WHERE company_id = c.id)
            WHERE c.symbol LIKE ? OR c.name LIKE ?
            ORDER BY c.symbol
            LIMIT ? OFFSET ?
        "#;
        
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query(search_query)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(CompanyWithLatestPrice {
                id: row.get("id"),
                symbol: row.get("symbol"),
                isin: row.get("isin"),
                series: row.get("series"),
                name: row.get("name"),
                latest_price: row.get("latest_price"),
                latest_date: row.get("latest_date"),
            });
        }
        
        // Get count
        let count_query = "SELECT COUNT(DISTINCT c.id) as count FROM companies c WHERE c.symbol LIKE ? OR c.name LIKE ?";
        let count_row = sqlx::query(count_query)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .fetch_one(&self.pool)
            .await?;
        let total: u64 = count_row.get::<i64, _>("count") as u64;
        
        Ok((results, total))
    }
    
    pub async fn get_market_overview(&self) -> Result<MarketOverview> {
        // Get basic counts
        let company_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM companies")
            .fetch_one(&self.pool).await?.get("count");
            
        let price_count: i64 = sqlx::query("SELECT COUNT(*) as count FROM daily_prices")
            .fetch_one(&self.pool).await?.get("count");
        
        // Get latest trading date
        let latest_date: Option<chrono::NaiveDate> = sqlx::query("SELECT MAX(trade_date) as max_date FROM daily_prices")
            .fetch_one(&self.pool).await?.get("max_date");
        
        // Get top gainers (limit to 5)
        let top_gainers = self.get_top_performers("gainers", 5).await?;
        
        // Get top losers (limit to 5)
        let top_losers = self.get_top_performers("losers", 5).await?;
        
        // Get most active stocks (by volume)
        let most_active = self.get_top_performers("volume", 5).await?;
        
        Ok(MarketOverview {
            total_companies: company_count,
            total_price_records: price_count,
            latest_trading_date: latest_date,
            top_gainers,
            top_losers,
            most_active,
        })
    }
    
    pub async fn get_top_performers(&self, metric: &str, limit: u32) -> Result<Vec<TopPerformer>> {
        let query = match metric {
            "gainers" => r#"
                SELECT c.symbol, c.series, dp.last_price as latest_price, dp.prev_close,
                       (dp.last_price - dp.prev_close) as price_change,
                       ((dp.last_price - dp.prev_close) / dp.prev_close * 100) as price_change_percent,
                       dp.total_traded_qty as volume
                FROM daily_prices dp
                JOIN companies c ON dp.company_id = c.id
                WHERE dp.trade_date = (SELECT MAX(trade_date) FROM daily_prices)
                AND dp.prev_close > 0
                ORDER BY price_change_percent DESC
                LIMIT ?
            "#,
            "losers" => r#"
                SELECT c.symbol, c.series, dp.last_price as latest_price, dp.prev_close,
                       (dp.last_price - dp.prev_close) as price_change,
                       ((dp.last_price - dp.prev_close) / dp.prev_close * 100) as price_change_percent,
                       dp.total_traded_qty as volume
                FROM daily_prices dp
                JOIN companies c ON dp.company_id = c.id
                WHERE dp.trade_date = (SELECT MAX(trade_date) FROM daily_prices)
                AND dp.prev_close > 0
                ORDER BY price_change_percent ASC
                LIMIT ?
            "#,
            "volume" => r#"
                SELECT c.symbol, c.series, dp.last_price as latest_price, dp.prev_close,
                       (dp.last_price - dp.prev_close) as price_change,
                       ((dp.last_price - dp.prev_close) / dp.prev_close * 100) as price_change_percent,
                       dp.total_traded_qty as volume
                FROM daily_prices dp
                JOIN companies c ON dp.company_id = c.id
                WHERE dp.trade_date = (SELECT MAX(trade_date) FROM daily_prices)
                ORDER BY dp.total_traded_qty DESC
                LIMIT ?
            "#,
            _ => return Err(anyhow::anyhow!("Invalid metric: {}", metric)),
        };
        
        let rows = sqlx::query(query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        
        let mut performers = Vec::new();
        for row in rows {
            performers.push(TopPerformer {
                symbol: row.get("symbol"),
                series: row.get("series"),
                latest_price: row.get("latest_price"),
                prev_close: row.get("prev_close"),
                price_change: row.get("price_change"),
                price_change_percent: row.get("price_change_percent"),
                volume: row.get("volume"),
            });
        }
        
        Ok(performers)
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

// Database models for API queries
#[derive(Debug)]
pub struct Company {
    pub id: i64,
    pub symbol: String,
    pub isin: String,
    pub series: String,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct StockPrice {
    pub id: i64,
    pub company_id: i64,
    pub symbol: String,
    pub trade_date: chrono::NaiveDate,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub prev_close: f64,
    pub total_traded_qty: i64,
    pub total_traded_value: f64,
    pub total_trades: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct CompanyWithLatestPrice {
    pub id: i64,
    pub symbol: String,
    pub isin: String,
    pub series: String,
    pub name: Option<String>,
    pub latest_price: Option<f64>,
    pub latest_date: Option<chrono::NaiveDate>,
}

#[derive(Debug)]
pub struct MarketOverview {
    pub total_companies: i64,
    pub total_price_records: i64,
    pub latest_trading_date: Option<chrono::NaiveDate>,
    pub top_gainers: Vec<TopPerformer>,
    pub top_losers: Vec<TopPerformer>,
    pub most_active: Vec<TopPerformer>,
}

#[derive(Debug)]
pub struct TopPerformer {
    pub symbol: String,
    pub series: String,
    pub latest_price: f64,
    pub prev_close: f64,
    pub price_change: f64,
    pub price_change_percent: f64,
    pub volume: i64,
}
