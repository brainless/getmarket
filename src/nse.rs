use reqwest::Client;
use anyhow::{Result, anyhow};
use chrono::{NaiveDate, Local, Datelike};
use tracing::{info, warn, error};
use std::io::Cursor;

pub struct NseClient {
    client: Client,
}

impl NseClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");
        
        NseClient { client }
    }

    pub async fn download_bhavcopy(&self, date: NaiveDate) -> Result<String> {
        let url = self.construct_bhavcopy_url(date)?;
        info!("Downloading bhavcopy from: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to download bhavcopy: HTTP {}", response.status()));
        }

        let content = response.text().await?;
        
        if content.trim().is_empty() {
            return Err(anyhow!("Downloaded file is empty"));
        }

        info!("Successfully downloaded bhavcopy for {}", date);
        Ok(content)
    }

    fn construct_bhavcopy_url(&self, date: NaiveDate) -> Result<String> {
        // NSE bhavcopy URL format: 
        // https://www.nseindia.com/content/historical/EQUITIES/{year}/{month}/cm{DD}{MMM}{YYYY}bhav.csv.zip
        // But this might be outdated, let's try the historical data URL first

        let day = format!("{:02}", date.day());
        let month_name = match date.month() {
            1 => "JAN", 2 => "FEB", 3 => "MAR", 4 => "APR", 5 => "MAY", 6 => "JUN",
            7 => "JUL", 8 => "AUG", 9 => "SEP", 10 => "OCT", 11 => "NOV", 12 => "DEC",
            _ => return Err(anyhow!("Invalid month")),
        };
        let month = format!("{:02}", date.month());
        let year = date.year();

        // Try the historical data URL first
        let url = format!(
            "https://www.nseindia.com/content/historical/EQUITIES/{}/{}/cm{}{}{}bhav.csv.zip",
            year, month, day, month_name, year
        );

        Ok(url)
    }

    pub fn get_trading_dates_in_range(&self, from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
        let mut dates = Vec::new();
        let mut current = from;

        while current <= to {
            // Skip weekends (Saturday = 6, Sunday = 0 in chrono)
            let weekday = current.weekday();
            if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun {
                dates.push(current);
            }
            current = current.succ_opt().unwrap_or(to);
        }

        dates
    }

    pub fn get_latest_trading_date(&self) -> NaiveDate {
        let today = Local::now().date_naive();
        let mut date = today;

        // Go back until we find a weekday (trading day)
        while date.weekday() == chrono::Weekday::Sat || date.weekday() == chrono::Weekday::Sun {
            date = date.pred_opt().unwrap_or(today);
        }

        date
    }
}

#[derive(Debug, Clone)]
pub struct StockRecord {
    pub symbol: String,
    pub series: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub last: f64,
    pub prevclose: f64,
    pub tottrdqty: i64,
    pub tottrdval: f64,
    pub timestamp: NaiveDate,
    pub totaltrades: i32,
    pub isin: String,
}

impl StockRecord {
    pub fn from_csv_record(record: &csv::StringRecord, date: NaiveDate) -> Result<Self> {
        // Expected CSV columns: SYMBOL,SERIES,OPEN,HIGH,LOW,CLOSE,LAST,PREVCLOSE,TOTTRDQTY,TOTTRDVAL,TIMESTAMP,TOTALTRADES,ISIN
        if record.len() < 13 {
            return Err(anyhow!("CSV record has insufficient columns: {}", record.len()));
        }

        Ok(StockRecord {
            symbol: record.get(0).unwrap_or("").to_string(),
            series: record.get(1).unwrap_or("").to_string(),
            open: record.get(2).unwrap_or("0").parse().unwrap_or(0.0),
            high: record.get(3).unwrap_or("0").parse().unwrap_or(0.0),
            low: record.get(4).unwrap_or("0").parse().unwrap_or(0.0),
            close: record.get(5).unwrap_or("0").parse().unwrap_or(0.0),
            last: record.get(6).unwrap_or("0").parse().unwrap_or(0.0),
            prevclose: record.get(7).unwrap_or("0").parse().unwrap_or(0.0),
            tottrdqty: record.get(8).unwrap_or("0").parse().unwrap_or(0),
            tottrdval: record.get(9).unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: date, // Use the passed date instead of parsing from CSV
            totaltrades: record.get(11).unwrap_or("0").parse().unwrap_or(0),
            isin: record.get(12).unwrap_or("").to_string(),
        })
    }
}

pub fn parse_csv_data(csv_content: &str, date: NaiveDate) -> Result<Vec<StockRecord>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(csv_content));
    let mut records = Vec::new();
    
    for result in reader.records() {
        match result {
            Ok(record) => {
                match StockRecord::from_csv_record(&record, date) {
                    Ok(stock_record) => {
                        // Filter out invalid records (empty symbols, etc.)
                        if !stock_record.symbol.is_empty() && stock_record.symbol != "-" {
                            records.push(stock_record);
                        }
                    },
                    Err(e) => {
                        warn!("Skipping invalid record: {}", e);
                    }
                }
            },
            Err(e) => {
                warn!("Error reading CSV record: {}", e);
            }
        }
    }
    
    info!("Parsed {} stock records for {}", records.len(), date);
    Ok(records)
}
