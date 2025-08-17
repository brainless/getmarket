use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use anyhow::Result;
use sqlx::Row;

mod database;
mod nse;
mod api;

use database::{Database, IngestionLogInsert};
use nse::{NseClient, parse_csv_data};
use tracing::{info, warn, error};

#[derive(Parser)]
#[command(name = "market-data")]
#[command(about = "A CLI tool for ingesting stock market data")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download and ingest stock market data
    Ingest {
        /// Data source to use
        #[arg(long, default_value = "nse")]
        source: String,
        
        /// Specific date to download (YYYY-MM-DD format)
        #[arg(long)]
        date: Option<String>,
        
        /// Start date for range download (YYYY-MM-DD format)
        #[arg(long)]
        from: Option<String>,
        
        /// End date for range download (YYYY-MM-DD format)
        #[arg(long)]
        to: Option<String>,
    },
    /// Show ingestion status and logs
    Status,
    /// Initialize the database
    InitDb {
        /// Database file path
        #[arg(long, default_value = "./market_data.db")]
        db_path: String,
    },
    /// Start the REST API server
    Serve {
        /// Database file path
        #[arg(long, default_value = "./market_data.db")]
        db_path: String,
        
        /// Server host address
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Server port
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Ingest { source, date, from, to } => {
            handle_ingest(source, date, from, to).await?
        },
        Commands::Status => {
            handle_status().await?
        },
        Commands::InitDb { db_path } => {
            handle_init_db(db_path).await?
        },
        Commands::Serve { db_path, host, port } => {
            handle_serve(db_path, host, port).await?
        },
    }
    
    Ok(())
}

async fn handle_ingest(source: String, date: Option<String>, from: Option<String>, to: Option<String>) -> Result<()> {
    info!("Starting data ingestion from source: {}", source);
    
    if source != "nse" {
        return Err(anyhow::anyhow!("Currently only 'nse' source is supported"));
    }
    
    // Connect to database
    let db = Database::new("sqlite://market_data.db").await?;
    let nse_client = NseClient::new();
    
    // Determine which dates to process
    let dates_to_process = if let Some(date_str) = date {
        if date_str == "today" {
            let today = nse_client.get_latest_trading_date();
            vec![today]
        } else {
            let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?;
            vec![parsed_date]
        }
    } else if let (Some(from_str), Some(to_str)) = (from, to) {
        let from_date = NaiveDate::parse_from_str(&from_str, "%Y-%m-%d")?;
        let to_date = NaiveDate::parse_from_str(&to_str, "%Y-%m-%d")?;
        nse_client.get_trading_dates_in_range(from_date, to_date)
    } else {
        let today = nse_client.get_latest_trading_date();
        vec![today]
    };
    
    info!("Processing {} dates", dates_to_process.len());
    
    for date in dates_to_process {
        match ingest_single_date(&db, &nse_client, date).await {
            Ok(count) => {
                println!("✅ Successfully ingested {} records for {}", count, date);
            },
            Err(e) => {
                error!("Failed to ingest data for {}: {}", date, e);
                println!("❌ Failed to ingest data for {}: {}", date, e);
            }
        }
    }
    
    Ok(())
}

async fn ingest_single_date(db: &Database, nse_client: &NseClient, date: NaiveDate) -> Result<usize> {
    let started_at = chrono::Local::now().naive_local();
    info!("Starting ingestion for date: {}", date);
    
    let mut log = IngestionLogInsert {
        source: "nse".to_string(),
        file_name: None,
        trade_date: Some(date),
        records_processed: Some(0),
        status: "failed".to_string(),
        error_message: None,
        started_at: Some(started_at),
        completed_at: chrono::Local::now().naive_local(),
    };
    
    let result = async {
        // Download bhavcopy
        println!("📥 Downloading data for {}...", date);
        let csv_content = nse_client.download_bhavcopy(date).await?;
        
        // Parse CSV data
        println!("🔍 Parsing CSV data...");
        let records = parse_csv_data(&csv_content, date)?;
        
        if records.is_empty() {
            return Err(anyhow::anyhow!("No valid records found in CSV data"));
        }
        
        // Store in database
        println!("💾 Storing {} records in database...", records.len());
        let stored_count = db.store_stock_records(&records).await?;
        
        Ok(stored_count)
    }.await;
    
    match result {
        Ok(count) => {
            log.records_processed = Some(count as i64);
            log.status = "success".to_string();
            log.completed_at = chrono::Local::now().naive_local();
        },
        Err(ref e) => {
            log.error_message = Some(e.to_string());
            log.completed_at = chrono::Local::now().naive_local();
        }
    }
    
    // Log the ingestion attempt
    if let Err(log_err) = db.log_ingestion(&log).await {
        warn!("Failed to log ingestion: {}", log_err);
    }
    
    result
}

async fn handle_status() -> Result<()> {
    println!("📊 Market Data Ingestion Status");
    println!("{}", "=".repeat(50));
    
    // Connect to database
    let db = Database::new("sqlite://market_data.db").await?;
    
    // Get recent ingestion logs
    match db.get_ingestion_logs(Some(10)).await {
        Ok(logs) => {
            if logs.is_empty() {
                println!("No ingestion logs found. Run 'market-data init-db' to initialize the database.");
                return Ok(());
            }
            
            println!("Recent ingestion attempts (last 10):");
            println!();
            
            for log in logs {
                let status_icon = match log.status.as_str() {
                    "success" => "✅",
                    "partial" => "⚠️",
                    "failed" => "❌",
                    _ => "❓"
                };
                
                println!(
                    "{} {} | {} | {} records | {}", 
                    status_icon,
                    log.trade_date
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or("N/A".to_string()),
                    log.source.to_uppercase(),
                    log.records_processed.unwrap_or(0),
                    log.completed_at.format("%Y-%m-%d %H:%M:%S")
                );
                
                if let Some(error) = &log.error_message {
                    println!("   Error: {}", error);
                }
            }
            
            // Get database statistics
            let company_count = get_table_count(&db, "companies").await?;
            let price_count = get_table_count(&db, "daily_prices").await?;
            
            println!();
            println!("Database Statistics:");
            println!("- Companies: {}", company_count);
            println!("- Price Records: {}", price_count);
        },
        Err(e) => {
            error!("Failed to fetch ingestion logs: {}", e);
            println!("❌ Failed to fetch status information: {}", e);
            println!("Make sure the database is initialized with: market-data init-db");
        }
    }
    
    Ok(())
}

async fn get_table_count(db: &Database, table_name: &str) -> Result<i64> {
    let query = format!("SELECT COUNT(*) as count FROM {}", table_name);
    let row = sqlx::query(&query)
        .fetch_one(&db.pool)
        .await?;
    Ok(row.get("count"))
}

async fn handle_init_db(db_path: String) -> Result<()> {
    println!("Initializing database at: {}", db_path);
    
    // Create the database URL
    let database_url = format!("sqlite://{}", db_path);
    
    // Connect to the database (will create file if it doesn't exist)
    let db = Database::new(&database_url).await?;
    
    // Initialize the schema
    db.initialize().await?;
    
    println!("✅ Database initialized successfully at: {}", db_path);
    println!("You can now use the 'ingest' command to start downloading data.");
    
    Ok(())
}

async fn handle_serve(db_path: String, host: String, port: u16) -> Result<()> {
    println!("Starting API server with database: {}", db_path);
    
    // Create the database URL
    let database_url = format!("sqlite://{}", db_path);
    
    // Connect to the database
    let db = Database::new(&database_url).await?;
    
    // Start the server
    api::start_server(host, port, db).await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
    
    Ok(())
}
