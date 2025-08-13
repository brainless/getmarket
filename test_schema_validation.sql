-- Test data and validation for the database schema
-- This script tests various constraints, triggers, and functionality

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- Test 1: Insert sample companies
INSERT INTO companies (symbol, isin, series, name) VALUES
    ('RELIANCE', 'INE002A01018', 'EQ', 'Reliance Industries Limited'),
    ('TCS', 'INE467B01029', 'EQ', 'Tata Consultancy Services Limited'),
    ('INFY', 'INE009A01021', 'EQ', 'Infosys Limited');

-- Test 2: Insert daily price data
INSERT INTO daily_prices (
    company_id, trade_date, open_price, high_price, low_price, 
    close_price, last_price, prev_close, total_traded_qty, 
    total_traded_value, total_trades
) SELECT 
    c.id, '2024-01-15', 2450.50, 2465.75, 2440.25, 
    2460.00, 2460.00, 2445.80, 1500000, 
    3690000000.00, 45000
FROM companies c WHERE c.symbol = 'RELIANCE';

INSERT INTO daily_prices (
    company_id, trade_date, open_price, high_price, low_price, 
    close_price, last_price, prev_close, total_traded_qty, 
    total_traded_value, total_trades
) SELECT 
    c.id, '2024-01-15', 3900.00, 3925.50, 3885.75, 
    3910.25, 3910.25, 3895.00, 850000, 
    3320000000.00, 32000
FROM companies c WHERE c.symbol = 'TCS';

-- Test 3: Log sample ingestion
INSERT INTO ingestion_log (
    source, file_name, trade_date, records_processed, 
    status, started_at, completed_at
) VALUES (
    'NSE', 'cm15JAN2024bhav.csv.zip', '2024-01-15', 
    2150, 'success', '2024-01-16 06:30:00', '2024-01-16 06:35:00'
);

-- Validation Queries

-- Query 1: Verify companies were inserted
SELECT 'Companies Count' as test_name, COUNT(*) as result FROM companies;

-- Query 2: Verify daily prices were inserted
SELECT 'Daily Prices Count' as test_name, COUNT(*) as result FROM daily_prices;

-- Query 3: Verify ingestion log was inserted
SELECT 'Ingestion Log Count' as test_name, COUNT(*) as result FROM ingestion_log;

-- Query 4: Test join query
SELECT 
    'Join Test' as test_name,
    c.symbol, 
    dp.close_price,
    dp.trade_date
FROM companies c
JOIN daily_prices dp ON c.id = dp.company_id
WHERE c.symbol = 'RELIANCE';

-- Query 5: Test updated_at trigger
-- First, record the current updated_at
SELECT 'Before Update' as test_name, symbol, updated_at FROM companies WHERE symbol = 'RELIANCE';

-- Update the company name (should trigger updated_at change)
UPDATE companies SET name = 'Reliance Industries Ltd.' WHERE symbol = 'RELIANCE';

-- Check if updated_at changed
SELECT 'After Update' as test_name, symbol, updated_at FROM companies WHERE symbol = 'RELIANCE';

-- Test constraint validations (these should fail)
-- Comment out to avoid errors, but these demonstrate constraint checking:

-- Test negative price constraint (should fail)
-- INSERT INTO daily_prices (company_id, trade_date, open_price, high_price, low_price, close_price) 
-- SELECT id, '2024-01-16', -100.00, 200.00, 50.00, 150.00 FROM companies WHERE symbol = 'TCS';

-- Test high < low constraint (should fail)  
-- INSERT INTO daily_prices (company_id, trade_date, open_price, high_price, low_price, close_price)
-- SELECT id, '2024-01-16', 100.00, 50.00, 200.00, 150.00 FROM companies WHERE symbol = 'TCS';

-- Test future date constraint (should fail)
-- INSERT INTO daily_prices (company_id, trade_date, open_price, high_price, low_price, close_price)
-- SELECT id, '2025-12-31', 100.00, 200.00, 50.00, 150.00 FROM companies WHERE symbol = 'TCS';

-- Final validation: Display all data
SELECT '=== FINAL DATA VALIDATION ===' as separator;
SELECT 'Companies:' as table_name;
SELECT * FROM companies;

SELECT 'Daily Prices:' as table_name;
SELECT * FROM daily_prices;

SELECT 'Ingestion Log:' as table_name;  
SELECT * FROM ingestion_log;
