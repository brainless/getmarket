# Market Data CLI

A Rust command-line application to ingest daily stock market data from NSE India (National Stock Exchange of India) and store it in a SQLite database. This is the MVP (Minimum Viable Product) implementation for a stock market data tracking system.

## 🚀 Features

- **NSE India Integration**: Download daily bhavcopy files directly from NSE
- **SQLite Database**: Efficient storage with proper schema design
- **Date Range Support**: Ingest single dates, date ranges, or the latest trading day
- **Robust Error Handling**: Graceful handling of network failures and data issues
- **Progress Tracking**: Comprehensive logging and status reporting
- **Weekend Filtering**: Automatically skips non-trading days
- **Duplicate Handling**: Uses upsert operations to handle duplicate data gracefully

## 📋 Prerequisites

- Rust 1.70+ and Cargo
- Internet connection for downloading NSE data
- SQLite (for manual database inspection, optional)

## 🛠️ Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd getmarket
```

2. Build the application:
```bash
cargo build --release
```

3. The binary will be available at `target/release/market-data`

## 📖 Usage

### Initialize Database

Before ingesting data, initialize the database schema:

```bash
cargo run -- init-db
# or specify custom database path
cargo run -- init-db --db-path /path/to/custom.db
```

### Ingest Stock Data

#### Download Today's Data
```bash
# Download data for the latest trading day
cargo run -- ingest --date today

# Or without specifying date (defaults to today)
cargo run -- ingest
```

#### Download Specific Date
```bash
cargo run -- ingest --date 2024-01-15
```

#### Download Date Range
```bash
cargo run -- ingest --from 2024-01-01 --to 2024-01-31
```

### Check Status

View ingestion history and database statistics:

```bash
cargo run -- status
```

Example output:
```
📊 Market Data Ingestion Status
==================================================
Recent ingestion attempts (last 10):

✅ 2025-01-15 | NSE | 1892 records | 2025-01-16 10:30:45
✅ 2025-01-14 | NSE | 1845 records | 2025-01-15 09:15:22
❌ 2025-01-13 | NSE | 0 records | 2025-01-14 11:20:10
   Error: Failed to download bhavcopy: HTTP 404

Database Statistics:
- Companies: 2156
- Price Records: 15420
```

### Help

View all available commands and options:

```bash
cargo run -- --help
cargo run -- ingest --help
```

## 🏗️ Architecture

### Database Schema

The application uses a SQLite database with three main tables:

#### `companies`
- `id`: Primary key
- `symbol`: Stock symbol (e.g., "RELIANCE")
- `isin`: International Securities Identification Number
- `series`: Trading series (e.g., "EQ")
- `name`: Company name (optional)
- `created_at`, `updated_at`: Timestamps

#### `daily_prices`
- `id`: Primary key
- `company_id`: Foreign key to companies table
- `trade_date`: Trading date
- `open_price`, `high_price`, `low_price`, `close_price`: OHLC prices
- `last_price`: Last traded price
- `prev_close`: Previous day's closing price
- `total_traded_qty`: Total quantity traded
- `total_traded_value`: Total value traded
- `total_trades`: Number of trades
- `created_at`: Timestamp

#### `ingestion_log`
- Tracks all ingestion attempts with status, error messages, and statistics

### Data Flow

1. **Download**: Fetch bhavcopy CSV files from NSE India
2. **Parse**: Extract stock records from CSV data
3. **Validate**: Filter invalid records and handle errors
4. **Store**: Upsert companies and daily price data
5. **Log**: Record ingestion status and statistics

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- CSV parsing and validation
- Date range calculations
- Weekend filtering
- Error handling scenarios

## 📊 Data Source

**NSE India (National Stock Exchange of India)**
- **URL Format**: `https://www.nseindia.com/content/historical/EQUITIES/{year}/{month}/cm{DD}{MMM}{YYYY}bhav.csv.zip`
- **Data Format**: CSV (bhavcopy files)
- **Update Frequency**: Daily (T+1, available next business day)
- **Authentication**: None required for historical data

### CSV Columns
The NSE bhavcopy files contain the following columns:
- `SYMBOL`: Stock symbol
- `SERIES`: Trading series
- `OPEN`, `HIGH`, `LOW`, `CLOSE`: Price data
- `LAST`: Last traded price
- `PREVCLOSE`: Previous closing price
- `TOTTRDQTY`: Total traded quantity
- `TOTTRDVAL`: Total traded value
- `TIMESTAMP`: Trading date
- `TOTALTRADES`: Number of trades
- `ISIN`: International Securities Identification Number

## 🔧 Configuration

The application uses sensible defaults but can be configured:

- **Database Path**: Default `./market_data.db`, customizable via `--db-path`
- **HTTP User Agent**: Set to mimic a standard browser
- **Date Format**: YYYY-MM-DD for all date inputs
- **Weekend Handling**: Automatically skips Saturday and Sunday

## 🚨 Error Handling

The application handles various error scenarios:

- **Network Failures**: Retry logic and graceful degradation
- **Invalid Data**: Skip malformed CSV records with warnings
- **Database Errors**: Transaction rollbacks and error reporting
- **File Access**: Clear error messages for file system issues

## 📈 Performance

- **Typical Performance**: Process 1000+ stock records in under 30 seconds
- **Memory Usage**: Efficient streaming of CSV data
- **Database**: Uses SQLite's upsert operations for optimal performance
- **Concurrency**: Async/await for non-blocking HTTP operations

## 🛣️ Roadmap

This MVP focuses on NSE India data ingestion. Future enhancements may include:

- [ ] Additional data sources (BSE, Yahoo Finance, etc.)
- [ ] Real-time data ingestion
- [ ] Web API interface
- [ ] Advanced analytics and calculations
- [ ] Data validation and anomaly detection
- [ ] Backup and restore functionality

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Troubleshooting

### Common Issues

**Database not found**
```bash
# Initialize the database first
cargo run -- init-db
```

**HTTP 404 errors**
- NSE data may not be available for weekends/holidays
- Check if the date is a valid trading day
- Some historical data may have different URL patterns

**Permission errors**
- Ensure write permissions for the database directory
- Check file system space availability

**Build errors**
- Ensure Rust 1.70+ is installed
- Run `cargo update` to update dependencies

### Debug Mode

Enable detailed logging by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -- ingest --date today
```

## 📞 Support

For questions, issues, or contributions, please:

1. Check existing [GitHub Issues](link-to-issues)
2. Create a new issue with detailed information
3. Include relevant logs and error messages
