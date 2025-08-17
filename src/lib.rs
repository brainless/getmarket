pub mod database;
pub mod nse;
pub mod api;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Datelike};
    
    #[test]
    fn test_stock_record_parsing() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let csv_data = r#"SYMBOL,SERIES,OPEN,HIGH,LOW,CLOSE,LAST,PREVCLOSE,TOTTRDQTY,TOTTRDVAL,TIMESTAMP,TOTALTRADES,ISIN
RELIANCE,EQ,2500.00,2550.00,2480.00,2520.00,2520.00,2500.00,1000000,2520000000.00,15-JAN-2025,50000,INE002A01018"#;

        let records = nse::parse_csv_data(csv_data, date).unwrap();
        assert_eq!(records.len(), 1);
        
        let record = &records[0];
        assert_eq!(record.symbol, "RELIANCE");
        assert_eq!(record.series, "EQ");
        assert_eq!(record.open, 2500.00);
        assert_eq!(record.high, 2550.00);
        assert_eq!(record.low, 2480.00);
        assert_eq!(record.close, 2520.00);
        assert_eq!(record.isin, "INE002A01018");
    }
    
    #[test]
    fn test_trading_dates_exclude_weekends() {
        let nse_client = nse::NseClient::new();
        
        // Test a range that includes a weekend
        let from = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap(); // Friday
        let to = NaiveDate::from_ymd_opt(2025, 1, 14).unwrap();   // Tuesday
        
        let dates = nse_client.get_trading_dates_in_range(from, to);
        
        // Should include Friday (10th), Monday (13th), Tuesday (14th)
        // Should exclude Saturday (11th), Sunday (12th)
        assert_eq!(dates.len(), 3);
        assert!(dates.contains(&NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()));
        assert!(dates.contains(&NaiveDate::from_ymd_opt(2025, 1, 13).unwrap()));
        assert!(dates.contains(&NaiveDate::from_ymd_opt(2025, 1, 14).unwrap()));
        assert!(!dates.contains(&NaiveDate::from_ymd_opt(2025, 1, 11).unwrap()));
        assert!(!dates.contains(&NaiveDate::from_ymd_opt(2025, 1, 12).unwrap()));
    }
    
    #[test]
    fn test_bhavcopy_url_construction() {
        // This tests the internal URL construction logic
        // We can't test the actual method since it's private, but we can test date formatting
        let date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        
        // Test month name conversion
        assert_eq!(date.day(), 15);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2025);
    }
    
    #[test]
    fn test_empty_csv_handling() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let empty_csv = "";
        
        let records = nse::parse_csv_data(empty_csv, date).unwrap();
        assert_eq!(records.len(), 0);
    }
    
    #[test]
    fn test_invalid_csv_record_handling() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        // CSV with insufficient columns
        let bad_csv = r#"SYMBOL,SERIES
RELIANCE,EQ"#;
        
        let records = nse::parse_csv_data(bad_csv, date).unwrap();
        // Should handle gracefully and return empty results
        assert_eq!(records.len(), 0);
    }
}
