# Database Schema Documentation

## Overview

This document provides comprehensive documentation for the GetMarket stock data database schema. The schema is designed to efficiently store and query daily stock market data from NSE (National Stock Exchange) India.

## Schema Design Principles

1. **Data Integrity**: Comprehensive constraints ensure data quality
2. **Performance**: Optimized indexes for common query patterns
3. **Scalability**: Designed to handle thousands of stocks over multiple years
4. **Audit Trail**: Complete logging of data ingestion processes
5. **Upsert Support**: Handles duplicate data gracefully with unique constraints

## Tables Overview

### 1. companies
Master table for storing stock/securities information.

**Columns:**
- `id` (INTEGER PRIMARY KEY): Unique identifier for each company
- `symbol` (TEXT NOT NULL UNIQUE): Stock symbol (e.g., "RELIANCE", "TCS")
- `isin` (TEXT): International Securities Identification Number (12 characters)
- `series` (TEXT): Trading series (e.g., "EQ" for Equity)
- `name` (TEXT): Full company name
- `created_at` (DATETIME): Record creation timestamp
- `updated_at` (DATETIME): Last update timestamp (auto-updated via trigger)

**Constraints:**
- Symbol must be non-empty
- ISIN must be NULL or at least 12 characters
- Unique constraint on symbol prevents duplicates

### 2. daily_prices
Daily OHLC (Open, High, Low, Close) and volume data for stocks.

**Columns:**
- `id` (INTEGER PRIMARY KEY): Unique identifier
- `company_id` (INTEGER NOT NULL): Foreign key to companies table
- `trade_date` (DATE NOT NULL): Trading date
- `open_price` (DECIMAL(10,2)): Opening price
- `high_price` (DECIMAL(10,2)): Highest price during the day
- `low_price` (DECIMAL(10,2)): Lowest price during the day
- `close_price` (DECIMAL(10,2)): Closing price
- `last_price` (DECIMAL(10,2)): Last traded price
- `prev_close` (DECIMAL(10,2)): Previous day's closing price
- `total_traded_qty` (BIGINT): Total quantity traded
- `total_traded_value` (DECIMAL(15,2)): Total value traded
- `total_trades` (INTEGER): Number of trades
- `created_at` (DATETIME): Record creation timestamp

**Constraints:**
- Foreign key relationship with companies table (CASCADE DELETE)
- Unique constraint on (company_id, trade_date) prevents duplicate data
- All price and volume fields must be non-negative
- High price must be >= low price
- Trade date cannot be in the future

### 3. ingestion_log
Audit log for tracking data ingestion processes.

**Columns:**
- `id` (INTEGER PRIMARY KEY): Unique identifier
- `source` (TEXT NOT NULL): Data source name (e.g., "NSE")
- `file_name` (TEXT): Name of the processed file
- `trade_date` (DATE): Trading date for the data
- `records_processed` (INTEGER): Number of records processed
- `status` (TEXT): Processing status ('success', 'partial', 'failed')
- `error_message` (TEXT): Error details if processing failed
- `started_at` (DATETIME): Processing start time
- `completed_at` (DATETIME): Processing completion time

## Indexes

The schema includes optimized indexes for common query patterns:

1. `idx_companies_symbol`: Fast company lookup by symbol
2. `idx_daily_prices_date`: Fast date-based queries
3. `idx_daily_prices_company_date`: Fast company-date lookups
4. `idx_ingestion_log_date`: Fast ingestion log queries by date

## Example Usage

### 1. Insert a New Company

```sql
INSERT INTO companies (symbol, isin, series, name) 
VALUES ('RELIANCE', 'INE002A01018', 'EQ', 'Reliance Industries Limited');
```

### 2. Insert Daily Price Data

```sql
INSERT INTO daily_prices (
    company_id, trade_date, open_price, high_price, low_price, 
    close_price, last_price, prev_close, total_traded_qty, 
    total_traded_value, total_trades
) 
SELECT 
    c.id, '2024-01-15', 2450.50, 2465.75, 2440.25, 
    2460.00, 2460.00, 2445.80, 1500000, 
    3690000000.00, 45000
FROM companies c 
WHERE c.symbol = 'RELIANCE';
```

### 3. Upsert Operation (Insert or Update)

```sql
INSERT INTO companies (symbol, isin, series, name) 
VALUES ('TCS', 'INE467B01029', 'EQ', 'Tata Consultancy Services Limited')
ON CONFLICT(symbol) DO UPDATE SET 
    isin = excluded.isin,
    series = excluded.series,
    name = excluded.name;
```

### 4. Query Recent Prices for a Stock

```sql
SELECT 
    c.symbol, c.name,
    dp.trade_date, dp.open_price, dp.high_price, 
    dp.low_price, dp.close_price, dp.total_traded_qty
FROM companies c
JOIN daily_prices dp ON c.id = dp.company_id
WHERE c.symbol = 'RELIANCE'
    AND dp.trade_date >= DATE('now', '-30 days')
ORDER BY dp.trade_date DESC;
```

### 5. Get Top Traded Stocks by Volume

```sql
SELECT 
    c.symbol, c.name,
    dp.close_price, dp.total_traded_qty, dp.total_traded_value
FROM companies c
JOIN daily_prices dp ON c.id = dp.company_id
WHERE dp.trade_date = '2024-01-15'
ORDER BY dp.total_traded_qty DESC
LIMIT 10;
```

### 6. Calculate Price Changes

```sql
SELECT 
    c.symbol,
    dp.close_price,
    dp.prev_close,
    ROUND(((dp.close_price - dp.prev_close) / dp.prev_close) * 100, 2) as change_percent
FROM companies c
JOIN daily_prices dp ON c.id = dp.company_id
WHERE dp.trade_date = '2024-01-15'
    AND dp.prev_close > 0
ORDER BY change_percent DESC;
```

### 7. Log Data Ingestion

```sql
INSERT INTO ingestion_log (
    source, file_name, trade_date, records_processed, 
    status, started_at, completed_at
) VALUES (
    'NSE', 'cm15JAN2024bhav.csv.zip', '2024-01-15', 
    2150, 'success', '2024-01-16 06:30:00', '2024-01-16 06:35:00'
);
```

### 8. Query Ingestion Statistics

```sql
SELECT 
    source,
    COUNT(*) as total_runs,
    SUM(CASE WHEN status = 'success' THEN 1 ELSE 0 END) as successful_runs,
    SUM(records_processed) as total_records,
    MIN(trade_date) as earliest_date,
    MAX(trade_date) as latest_date
FROM ingestion_log
GROUP BY source;
```

## Migration and Maintenance

### Schema Creation
To create the database schema:
```bash
sqlite3 market_data.db < database_schema.sql
```

### Backup
```bash
sqlite3 market_data.db ".backup backup_$(date +%Y%m%d).db"
```

### Vacuum and Optimize
```sql
VACUUM;
ANALYZE;
```

## Performance Considerations

1. **Bulk Inserts**: Use transactions for inserting large batches of data
2. **Index Usage**: Query patterns are optimized for the existing indexes
3. **Data Archival**: Consider partitioning or archiving old data for very large datasets
4. **Memory Settings**: For large imports, increase SQLite cache_size and page_size

## Data Validation Rules

The schema enforces several data validation rules:

1. **Price Integrity**: All prices must be non-negative, high >= low
2. **Date Validation**: Trade dates cannot be in the future
3. **Volume Validation**: All volume and trade count fields must be non-negative
4. **Symbol Validation**: Stock symbols must be non-empty
5. **ISIN Validation**: ISIN codes must be properly formatted (12+ characters)

## Trigger Behavior

The `update_companies_updated_at` trigger automatically updates the `updated_at` field in the companies table whenever any other field is modified, ensuring accurate audit trails.

---

This schema provides a robust foundation for storing and querying stock market data, with built-in data integrity, performance optimization, and comprehensive audit capabilities.
