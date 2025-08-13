# Implement Actix Web REST API for Stock Data Browsing

## Overview

This issue covers implementing a REST API using Actix Web to provide HTTP endpoints for browsing the downloaded NSE stock market data. This will serve as the backend for the web application and enable programmatic access to the stock data.

## Requirements

### Core API Endpoints

#### 1. Companies API
- **GET /api/companies** - List all companies with pagination
  - Query params: `page`, `limit`, `search`, `series`
  - Response: Paginated list of companies with metadata

#### 2. Stock Prices API  
- **GET /api/companies/{symbol}/prices** - Get price history for a company
  - Query params: `from_date`, `to_date`, `limit`
  - Response: Time series data with OHLC values
  
- **GET /api/prices/latest** - Get latest prices for all stocks
  - Query params: `date`, `series`, `limit`
  - Response: Latest trading data snapshot

#### 3. Market Data API
- **GET /api/market/overview** - Market overview and statistics
  - Response: Trading volume, market cap, top gainers/losers
  
- **GET /api/market/top-performers** - Top performing stocks
  - Query params: `period`, `metric` (volume, price_change, etc.)
  - Response: Ranked list of top performers

#### 4. Search & Filter API
- **GET /api/search** - Search stocks by symbol or company name
  - Query params: `q` (query), `limit`
  - Response: Matching companies and symbols
  
- **GET /api/filter** - Advanced filtering capabilities
  - Query params: Various filters (price_range, volume_range, series, etc.)
  - Response: Filtered stock results

#### 5. Analytics API
- **GET /api/analytics/price-changes** - Price change analysis
  - Query params: `period`, `symbols`
  - Response: Price change statistics and trends
  
- **GET /api/analytics/volume-analysis** - Volume analysis
  - Query params: `date_range`, `symbols`
  - Response: Volume patterns and insights

### Technical Requirements

#### Framework & Dependencies
- **Actix Web 4.x** - Main web framework
- **Serde** - JSON serialization/deserialization
- **SQLx** - Database integration (reuse existing connection)
- **Chrono** - Date/time handling
- **Tokio** - Async runtime
- **Anyhow** - Error handling
- **Tracing** - Structured logging
- **Actix-CORS** - CORS middleware for web app integration

#### API Features
- **JSON responses** with consistent error format
- **Pagination** for large datasets
- **Query parameter validation** 
- **Rate limiting** for API protection
- **Comprehensive error handling** with proper HTTP status codes
- **OpenAPI/Swagger documentation** generation
- **Health check endpoint** (/health)
- **Metrics endpoint** for monitoring (/metrics)

#### Data Models & Serialization
- **Response DTOs** optimized for API consumption
- **Query parameter structs** with validation
- **Consistent pagination wrapper**
- **Error response models**
- **Date formatting** (ISO 8601)

#### Performance Optimizations
- **Database connection pooling** (reuse existing SQLx pool)
- **Query optimization** with proper indexes
- **Response caching** for frequently accessed data
- **Streaming responses** for large datasets
- **Database query batching** where applicable

### API Design Principles

#### RESTful Design
- Proper HTTP methods and status codes
- Resource-based URLs
- Stateless interactions
- Consistent naming conventions

#### Security
- Input validation and sanitization
- SQL injection prevention (SQLx handles this)
- Rate limiting middleware
- CORS configuration for web app
- Request size limits

#### Documentation
- OpenAPI 3.0 specification
- Interactive Swagger UI
- API usage examples
- Response schema documentation

## Implementation Plan

### Phase 1: Core Infrastructure
- [ ] Set up Actix Web server structure
- [ ] Configure database integration with existing SQLx pool
- [ ] Implement basic middleware (CORS, logging, error handling)
- [ ] Create base response models and error handling
- [ ] Add health check and metrics endpoints

### Phase 2: Basic Data APIs
- [ ] Implement companies listing API
- [ ] Implement stock prices API for individual symbols
- [ ] Add pagination and basic filtering
- [ ] Implement search functionality

### Phase 3: Advanced Features
- [ ] Market overview and statistics APIs
- [ ] Top performers and analytics endpoints
- [ ] Advanced filtering and sorting
- [ ] Response caching implementation

### Phase 4: Documentation & Testing
- [ ] OpenAPI documentation generation
- [ ] Integration tests for all endpoints
- [ ] Performance testing and optimization
- [ ] API usage documentation

## File Structure

```
src/
├── main.rs              # CLI and server startup
├── api/
│   ├── mod.rs          # API module exports
│   ├── routes/         # Route handlers
│   │   ├── companies.rs
│   │   ├── prices.rs
│   │   ├── market.rs
│   │   └── search.rs
│   ├── models/         # Response DTOs and request models
│   │   ├── responses.rs
│   │   ├── requests.rs
│   │   └── pagination.rs
│   ├── middleware/     # Custom middleware
│   └── handlers/       # Business logic handlers
├── database.rs         # Existing database module (extend)
└── lib.rs             # Library exports
```

## Configuration

### CLI Integration
Extend the existing CLI to include a server subcommand:
```bash
# Start the API server
market-data serve --port 8080 --host 0.0.0.0

# With specific database
market-data serve --db-path ./custom_market_data.db --port 3000
```

### Environment Configuration
- Database path configuration
- Server host and port settings  
- CORS origin configuration
- Rate limiting settings
- Cache configuration

## Testing Strategy

### Unit Tests
- Database query functions
- Data model serialization
- Business logic validation
- Error handling scenarios

### Integration Tests  
- Full API endpoint testing
- Database integration testing
- Pagination and filtering logic
- Cross-endpoint data consistency

### Performance Tests
- Load testing with multiple concurrent requests
- Database query performance validation
- Memory usage optimization
- Response time benchmarking

## Success Criteria

- [ ] All API endpoints implemented and functional
- [ ] Comprehensive test coverage (>80%)
- [ ] API documentation available via Swagger UI
- [ ] Performance benchmarks meet requirements (<100ms average response time)
- [ ] Integration with existing CLI and database works seamlessly
- [ ] CORS properly configured for web app integration
- [ ] Error handling provides meaningful feedback
- [ ] API follows REST best practices

## Dependencies on Other Issues

This issue depends on:
- Issue #2 (Database Schema) - ✅ Completed
- Issue #1 (NSE Data Ingestion) - ✅ Completed

This issue enables:
- Web application frontend (next issue)
- Third-party integrations
- Mobile app development (future)

## Timeline Estimate

- **Phase 1**: 2-3 days
- **Phase 2**: 3-4 days  
- **Phase 3**: 2-3 days
- **Phase 4**: 2 days
- **Total**: ~10-12 days

---

This REST API will provide a robust foundation for the web application and enable future integrations with mobile apps and third-party services.
