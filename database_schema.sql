-- Database schema for NSE stock market data ingestion
-- This schema is designed to store daily stock data from NSE bhavcopy files

-- Companies/Securities master table
CREATE TABLE companies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL UNIQUE,
    isin TEXT,
    series TEXT,
    name TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    -- Additional constraints
    CHECK(length(symbol) > 0),
    CHECK(isin IS NULL OR length(isin) >= 12)
);

-- Daily stock prices table
CREATE TABLE daily_prices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id INTEGER NOT NULL,
    trade_date DATE NOT NULL,
    open_price DECIMAL(10,2),
    high_price DECIMAL(10,2),
    low_price DECIMAL(10,2),
    close_price DECIMAL(10,2),
    last_price DECIMAL(10,2),
    prev_close DECIMAL(10,2),
    total_traded_qty BIGINT,
    total_traded_value DECIMAL(15,2),
    total_trades INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    UNIQUE(company_id, trade_date),
    -- Additional constraints for data integrity
    CHECK(open_price >= 0),
    CHECK(high_price >= 0),
    CHECK(low_price >= 0),
    CHECK(close_price >= 0),
    CHECK(last_price >= 0),
    CHECK(prev_close >= 0),
    CHECK(total_traded_qty >= 0),
    CHECK(total_traded_value >= 0),
    CHECK(total_trades >= 0),
    CHECK(high_price >= low_price)
);

-- Data ingestion log table
CREATE TABLE ingestion_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    file_name TEXT,
    trade_date DATE,
    records_processed INTEGER,
    status TEXT CHECK(status IN ('success', 'partial', 'failed')),
    error_message TEXT,
    started_at DATETIME,
    completed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX idx_companies_symbol ON companies(symbol);
CREATE INDEX idx_daily_prices_date ON daily_prices(trade_date);
CREATE INDEX idx_daily_prices_company_date ON daily_prices(company_id, trade_date);
CREATE INDEX idx_ingestion_log_date ON ingestion_log(trade_date);

-- Trigger to automatically update the updated_at timestamp
CREATE TRIGGER update_companies_updated_at 
    AFTER UPDATE ON companies
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE companies SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
