use reqwest::Client;
use anyhow::{Result, anyhow};
use chrono::{NaiveDate, Local, Datelike};
use tracing::{info, warn, error};
use std::io::Cursor;
use flate2::read::GzDecoder;
use std::io::Read;

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
        // Try multiple URL strategies since NSE has changed their API structure
        let urls = self.construct_bhavcopy_urls(date)?;
        
        for (i, url) in urls.iter().enumerate() {
            info!("Attempting strategy {} - Downloading bhavcopy from: {}", i + 1, url);
            println!("🌐 Trying URL strategy {} of {}: {}", i + 1, urls.len(), url);
            
            let response = self.client
                .get(url)
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Accept-Encoding", "gzip, deflate")
                .header("Connection", "keep-alive")
                .header("Upgrade-Insecure-Requests", "1")
                .send()
                .await?;
            
            if response.status().is_success() {
                let bytes = response.bytes().await?;
                
                // Try to decode as UTF-8 first
                let content = if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    text
                } else {
                    // If that fails, try gzip decompression
                    let mut decoder = GzDecoder::new(&bytes[..]);
                    let mut decompressed = String::new();
                    match decoder.read_to_string(&mut decompressed) {
                        Ok(_) => decompressed,
                        Err(_) => {
                            warn!("Failed to decompress content from URL: {}", url);
                            continue;
                        }
                    }
                };
                
                if !content.trim().is_empty() && content.len() > 100 && content.lines().count() > 1 {
                    info!("Successfully downloaded bhavcopy for {} using strategy {}", date, i + 1);
                    println!("✅ Successfully downloaded {} bytes ({} lines)", content.len(), content.lines().count());
                    return Ok(content);
                } else {
                    warn!("Downloaded file is empty, too small, or has no data from URL: {} (length: {}, lines: {})", url, content.len(), content.lines().count());
                }
            } else {
                warn!("HTTP {} from URL: {}", response.status(), url);
            }
        }
        
        Err(anyhow!("Failed to download bhavcopy from any available source. All {} URL strategies failed.", urls.len()))
    }

    fn construct_bhavcopy_urls(&self, date: NaiveDate) -> Result<Vec<String>> {
        let day = format!("{:02}", date.day());
        let month_name = match date.month() {
            1 => "JAN", 2 => "FEB", 3 => "MAR", 4 => "APR", 5 => "MAY", 6 => "JUN",
            7 => "JUL", 8 => "AUG", 9 => "SEP", 10 => "OCT", 11 => "NOV", 12 => "DEC",
            _ => return Err(anyhow!("Invalid month")),
        };
        let month = format!("{:02}", date.month());
        let year = date.year();
        
        let mut urls = Vec::new();
        
        // Strategy 1: New NSE API format (archives/equities/bhavcopy)
        urls.push(format!(
            "https://archives.nseindia.com/products/content/sec_bhavdata_full_{:02}{:02}{}.csv",
            day.parse::<u32>().unwrap_or(1), date.month(), year
        ));
        
        // Strategy 2: Alternative archives format
        urls.push(format!(
            "https://www.nseindia.com/archives/equities/bhavcopy/pr/pr{:02}{:02}{}.zip",
            day.parse::<u32>().unwrap_or(1), date.month(), (year % 100)
        ));
        
        // Strategy 3: Original historical format (legacy)
        urls.push(format!(
            "https://www.nseindia.com/content/historical/EQUITIES/{}/{}/cm{}{}{}bhav.csv.zip",
            year, month, day, month_name, year
        ));
        
        // Strategy 4: Direct CSV download
        urls.push(format!(
            "https://www.nseindia.com/products/dynaContent/common/productsSymbolMapping/EQUITY_L.csv"
        ));
        
        // Strategy 5: Modern NSE API endpoint (if available)
        urls.push(format!(
            "https://www.nseindia.com/api/historical/cm/equity?from={}&to={}",
            date.format("%d-%m-%Y"),
            date.format("%d-%m-%Y")
        ));
        
        Ok(urls)
    }
    
    fn construct_bhavcopy_url(&self, date: NaiveDate) -> Result<String> {
        // Keep this method for backward compatibility
        let urls = self.construct_bhavcopy_urls(date)?;
        Ok(urls.into_iter().next().unwrap_or_default())
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
        // Handle both old and new NSE CSV formats
        if record.len() >= 15 {
            // New NSE format: SYMBOL, SERIES, DATE1, PREV_CLOSE, OPEN_PRICE, HIGH_PRICE, LOW_PRICE, LAST_PRICE, CLOSE_PRICE, AVG_PRICE, TTL_TRD_QNTY, TURNOVER_LACS, NO_OF_TRADES, DELIV_QTY, DELIV_PER
            Self::from_new_csv_format(record, date)
        } else if record.len() >= 13 {
            // Old NSE format: SYMBOL,SERIES,OPEN,HIGH,LOW,CLOSE,LAST,PREVCLOSE,TOTTRDQTY,TOTTRDVAL,TIMESTAMP,TOTALTRADES,ISIN
            Self::from_old_csv_format(record, date)
        } else {
            Err(anyhow!("CSV record has insufficient columns: {}", record.len()))
        }
    }
    
    fn from_new_csv_format(record: &csv::StringRecord, date: NaiveDate) -> Result<Self> {
        // New NSE format mapping:
        // 0: SYMBOL, 1: SERIES, 2: DATE1, 3: PREV_CLOSE, 4: OPEN_PRICE, 5: HIGH_PRICE, 
        // 6: LOW_PRICE, 7: LAST_PRICE, 8: CLOSE_PRICE, 9: AVG_PRICE, 10: TTL_TRD_QNTY, 
        // 11: TURNOVER_LACS, 12: NO_OF_TRADES, 13: DELIV_QTY, 14: DELIV_PER
        
        let symbol = record.get(0).unwrap_or("").trim().to_string();
        let series = record.get(1).unwrap_or("").trim().to_string();
        
        // Skip invalid records
        if symbol.is_empty() || symbol == "-" || series.is_empty() {
            return Err(anyhow!("Invalid symbol or series: '{}', '{}'", symbol, series));
        }
        
        // Convert turnover from lakhs to actual value
        let turnover_lacs: f64 = record.get(11).unwrap_or("0").trim().parse().unwrap_or(0.0);
        let tottrdval = turnover_lacs * 100000.0; // Convert lakhs to rupees
        
        Ok(StockRecord {
            symbol,
            series,
            open: record.get(4).unwrap_or("0").trim().parse().unwrap_or(0.0),
            high: record.get(5).unwrap_or("0").trim().parse().unwrap_or(0.0),
            low: record.get(6).unwrap_or("0").trim().parse().unwrap_or(0.0),
            close: record.get(8).unwrap_or("0").trim().parse().unwrap_or(0.0),
            last: record.get(7).unwrap_or("0").trim().parse().unwrap_or(0.0),
            prevclose: record.get(3).unwrap_or("0").trim().parse().unwrap_or(0.0),
            tottrdqty: record.get(10).unwrap_or("0").trim().parse().unwrap_or(0),
            tottrdval,
            timestamp: date,
            totaltrades: record.get(12).unwrap_or("0").trim().parse().unwrap_or(0),
            isin: String::new(), // Not available in new format
        })
    }
    
    fn from_old_csv_format(record: &csv::StringRecord, date: NaiveDate) -> Result<Self> {
        // Old format: SYMBOL,SERIES,OPEN,HIGH,LOW,CLOSE,LAST,PREVCLOSE,TOTTRDQTY,TOTTRDVAL,TIMESTAMP,TOTALTRADES,ISIN
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
            timestamp: date,
            totaltrades: record.get(11).unwrap_or("0").parse().unwrap_or(0),
            isin: record.get(12).unwrap_or("").to_string(),
        })
    }
}

pub fn parse_csv_data(csv_content: &str, date: NaiveDate) -> Result<Vec<StockRecord>> {
    let mut reader = csv::Reader::from_reader(Cursor::new(csv_content));
    let mut records = Vec::new();
    let mut total_processed = 0;
    let mut skipped_count = 0;
    
    // Debug: Print first few lines to understand the format
    let lines: Vec<&str> = csv_content.lines().take(3).collect();
    for (i, line) in lines.iter().enumerate() {
        println!("📋 CSV Line {}: {}", i, line);
    }
    
    for result in reader.records() {
        total_processed += 1;
        match result {
            Ok(record) => {
                // Debug first few records
                if total_processed <= 3 {
                    println!("🔍 Record {} has {} columns: {:?}", total_processed, record.len(), record);
                }
                
                match StockRecord::from_csv_record(&record, date) {
                    Ok(stock_record) => {
                        // Filter out invalid records (empty symbols, etc.)
                        if !stock_record.symbol.is_empty() && stock_record.symbol != "-" {
                            if records.len() < 3 {
                                println!("✅ Parsed record {}: {} ({})", records.len() + 1, stock_record.symbol, stock_record.series);
                            }
                            records.push(stock_record);
                        } else {
                            skipped_count += 1;
                        }
                    },
                    Err(e) => {
                        skipped_count += 1;
                        if skipped_count <= 5 {
                            warn!("Skipping invalid record {}: {}", total_processed, e);
                        }
                    }
                }
            },
            Err(e) => {
                warn!("Error reading CSV record {}: {}", total_processed, e);
            }
        }
    }
    
    info!("Parsed {} stock records from {} total records ({} skipped) for {}", records.len(), total_processed, skipped_count, date);
    println!("📊 Final result: {} valid records out of {} processed", records.len(), total_processed);
    Ok(records)
}
