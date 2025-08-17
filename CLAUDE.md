# CLAUDE.md - GetMarket Project

## Project Overview
GetMarket is a Rust CLI application for ingesting and tracking stock market data from Indian exchanges, starting with NSE (National Stock Exchange). This is an MVP implementation focused on daily bhavcopy data ingestion and SQLite storage.

# Development Workflow
- Create a new branch for each task
- Branch names should start with `feature/`, `chore/` or `fix/`
- Please add tests for any new features added, particularly integration tests
- Please run formatters, linters and tests before committing changes
- When finished please commit and push to the new branch
- Please mention GitHub issue if provided
- After working on an issue from GitHub, update issue's tasks and open PR

**Status**: MVP Development Phase
**Language**: Rust (Edition 2024)
**Database**: SQLite
**Primary Data Source**: NSE India

## Vision & Goals
- Track listed companies, stocks, articles, PR from publicly available data
- Users can create portfolios, manage tracking, and get alerts
- Start with Indian stock market data, expand to other sources
- Build foundation for comprehensive market data platform

## Project Structure

### Core Files
```
getmarket/
├── src/
│   ├── main.rs          # CLI interface & command handling
│   ├── database.rs      # SQLite operations & schema management
│   ├── nse.rs          # NSE data fetching & CSV parsing
│   └── lib.rs          # Library entry point
├── project/
│   └── OVERVIEW.md      # Data sources research & planning
├── database_schema.sql  # SQLite schema definition
├── issue_1.txt         # Current GitHub issue details
├── Cargo.toml          # Dependencies & project config
└── README.md           # User documentation
```

### Key Dependencies
- **clap**: CLI argument parsing
- **tokio + reqwest**: Async HTTP operations
- **csv**: NSE bhavcopy file parsing
- **sqlx**: Database operations with compile-time query validation
- **chrono**: Date/time handling
- **tracing**: Logging and debugging

## Database Schema

### Core Tables
1. **companies** - Master table for stock symbols and metadata
2. **daily_prices** - Daily OHLC and volume data with company FK
3. **ingestion_log** - Track data ingestion attempts and status

### Key Features
- Upsert operations for handling duplicates
- Foreign key relationships
- Indexed queries for performance
- Audit trails for data ingestion

## CLI Interface

### Available Commands
```bash
# Initialize database schema
market-data init-db [--db-path PATH]

# Download and ingest data
market-data ingest [--source nse] [--date YYYY-MM-DD|today]
market-data ingest --from YYYY-MM-DD --to YYYY-MM-DD

# View ingestion status and statistics
market-data status
```

### Data Flow
1. **Download**: Fetch bhavcopy CSV from NSE India
2. **Parse**: Extract stock records with validation
3. **Store**: Upsert companies and daily price data
4. **Log**: Record ingestion status and statistics

## Current Issues & Development Status

### Open Issues
**Issue #1: Implement Rust CLI for NSE India Stock Data Ingestion - MVP**
- **Status**: In Progress (assigned to brainless)
- **Scope**: Core MVP functionality for NSE data ingestion

#### Requirements Progress
- [x] Project structure and dependencies setup
- [x] Database schema design and implementation
- [x] CLI interface with clap
- [x] NSE client for bhavcopy downloads
- [x] CSV parsing with error handling
- [x] SQLite storage with upsert operations
- [x] Comprehensive logging and status reporting
- [ ] Unit tests for core parsing and database logic
- [ ] Integration tests with sample CSV data
- [ ] Performance validation (process 1000+ stocks in <30s)
- [ ] Full error handling validation

#### Key Implementation Details
- **NSE URL Pattern**: `https://www.nseindia.com/content/historical/EQUITIES/{year}/{month}/cm{DD}{MMM}{YYYY}bhav.csv.zip`
- **Authentication**: None required for historical data
- **Weekend Handling**: Automatically filters non-trading days
- **Error Handling**: Network failures, invalid CSV, database errors
- **Performance Target**: <30 seconds for 1000+ stock records

## Development Guidelines

### Code Organization
- **main.rs**: CLI commands and high-level orchestration
- **database.rs**: All SQLite operations, schema management
- **nse.rs**: NSE-specific data fetching and parsing logic
- Modular design for easy addition of new data sources

### Error Handling Strategy
- Use `anyhow` for error propagation
- Comprehensive logging with `tracing`
- Graceful handling of network failures
- Transaction rollbacks on database errors
- User-friendly error messages

### Future Extensibility
- Abstract interfaces for multiple data sources
- Plugin architecture for new exchanges
- Configurable retry logic and timeouts
- Support for real-time data ingestion

## Immediate Next Steps
1. Complete unit test coverage
2. Add integration tests with sample data
3. Validate performance requirements
4. Handle edge cases and error scenarios
5. Consider BSE India as next data source

## Data Sources (Planned)
- **NSE India** ✅ (Current focus)
- **BSE India** 📋 (Next target)
- **Yahoo Finance** 📋 (API integration)
- **Kaggle Datasets** 📋 (Historical data)
- **Upstox API** 📋 (Real-time capabilities)

## Performance Characteristics
- **Target**: Process 1000+ stocks in <30 seconds
- **Memory**: Streaming CSV processing for efficiency
- **Database**: SQLite with optimized upsert operations
- **Concurrency**: Async/await for non-blocking HTTP operations

---
*This documentation reflects the current state of the MVP implementation focusing on NSE India data ingestion.*
