# Market Data REST API Documentation

This document describes the REST API endpoints implemented for the Market Data application.

## Base URL
When running the server locally: `http://127.0.0.1:8080/api`

## Starting the Server

```bash
# Start with default settings (localhost:8080)
./target/release/market-data serve

# Start with custom host and port
./target/release/market-data serve --host 0.0.0.0 --port 3000

# Start with custom database
./target/release/market-data serve --db-path ./custom_market_data.db --port 8080
```

## API Endpoints

### 1. Health Check
Check if the API server is running and database is connected.

**GET** `/api/health`

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-01-17T10:30:00Z",
    "database_status": "connected",
    "version": "0.1.0"
  },
  "error": null,
  "timestamp": "2025-01-17T10:30:00Z"
}
```

### 2. Companies API

#### List Companies
Get a paginated list of all companies.

**GET** `/api/companies`

**Query Parameters:**
- `page` (optional, default: 1) - Page number
- `limit` (optional, default: 50) - Items per page (max: 1000)
- `search` (optional) - Search by symbol or company name
- `series` (optional) - Filter by trading series (e.g., "EQ")

**Example Request:**
```
GET /api/companies?page=1&limit=10&search=RELIANCE&series=EQ
```

**Response:**
```json
{
  "success": true,
  "data": {
    "data": [
      {
        "id": 1,
        "symbol": "RELIANCE",
        "isin": "INE002A01018",
        "series": "EQ",
        "name": null,
        "created_at": "2025-01-15T10:30:00",
        "updated_at": "2025-01-15T10:30:00"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 10,
      "total": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  },
  "error": null,
  "timestamp": "2025-01-17T10:30:00Z"
}
```

### 3. Stock Prices API

#### Get Company Price History
Get historical price data for a specific company.

**GET** `/api/companies/{symbol}/prices`

**Path Parameters:**
- `symbol` - Stock symbol (e.g., "RELIANCE")

**Query Parameters:**
- `page` (optional, default: 1) - Page number
- `limit` (optional, default: 50) - Items per page
- `from_date` (optional) - Start date (YYYY-MM-DD)
- `to_date` (optional) - End date (YYYY-MM-DD)

**Example Request:**
```
GET /api/companies/RELIANCE/prices?from_date=2025-01-01&to_date=2025-01-15&limit=5
```

**Response:**
```json
{
  "success": true,
  "data": {
    "data": [
      {
        "id": 1,
        "company_id": 1,
        "symbol": "RELIANCE",
        "trade_date": "2025-01-15",
        "open_price": 2500.0,
        "high_price": 2550.0,
        "low_price": 2480.0,
        "close_price": 2520.0,
        "last_price": 2520.0,
        "prev_close": 2500.0,
        "total_traded_qty": 1000000,
        "total_traded_value": 2520000000.0,
        "total_trades": 50000,
        "created_at": "2025-01-15T10:30:00"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 5,
      "total": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  },
  "error": null,
  "timestamp": "2025-01-17T10:30:00Z"
}
```

#### Get Latest Prices
Get the latest trading prices for all stocks.

**GET** `/api/prices/latest`

**Query Parameters:**
- `page` (optional, default: 1) - Page number
- `limit` (optional, default: 50) - Items per page
- `date` (optional) - Specific date (YYYY-MM-DD), defaults to latest available
- `series` (optional) - Filter by trading series

**Example Request:**
```
GET /api/prices/latest?date=2025-01-15&series=EQ&limit=10
```

### 4. Search API

#### Search Companies
Search for companies by symbol or name.

**GET** `/api/search`

**Query Parameters:**
- `q` (required) - Search query
- `page` (optional, default: 1) - Page number
- `limit` (optional, default: 50) - Items per page

**Example Request:**
```
GET /api/search?q=TATA&limit=5
```

**Response:**
```json
{
  "success": true,
  "data": {
    "data": [
      {
        "symbol": "TATA",
        "isin": "INE155A01022",
        "series": "EQ",
        "name": "Tata Motors Ltd",
        "latest_price": 1250.0,
        "latest_date": "2025-01-15"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 5,
      "total": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  },
  "error": null,
  "timestamp": "2025-01-17T10:30:00Z"
}
```

### 5. Market Data API

#### Market Overview
Get market overview including statistics and top performers.

**GET** `/api/market/overview`

**Response:**
```json
{
  "success": true,
  "data": {
    "total_companies": 2156,
    "total_price_records": 15420,
    "latest_trading_date": "2025-01-15",
    "top_gainers": [
      {
        "symbol": "STOCKA",
        "series": "EQ",
        "latest_price": 1100.0,
        "prev_close": 1000.0,
        "price_change": 100.0,
        "price_change_percent": 10.0,
        "volume": 500000
      }
    ],
    "top_losers": [
      {
        "symbol": "STOCKB",
        "series": "EQ",
        "latest_price": 900.0,
        "prev_close": 1000.0,
        "price_change": -100.0,
        "price_change_percent": -10.0,
        "volume": 300000
      }
    ],
    "most_active": [
      {
        "symbol": "STOCKC",
        "series": "EQ",
        "latest_price": 1050.0,
        "prev_close": 1000.0,
        "price_change": 50.0,
        "price_change_percent": 5.0,
        "volume": 2000000
      }
    ]
  },
  "error": null,
  "timestamp": "2025-01-17T10:30:00Z"
}
```

## Error Responses

All API endpoints return a consistent error format:

```json
{
  "success": false,
  "data": null,
  "error": "Database error: Connection failed",
  "timestamp": "2025-01-17T10:30:00Z"
}
```

Common HTTP status codes:
- `200 OK` - Successful request
- `400 Bad Request` - Invalid query parameters
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Database or server error

## Testing the API

You can test the API using curl:

```bash
# Check health
curl http://127.0.0.1:8080/api/health

# Get companies
curl "http://127.0.0.1:8080/api/companies?limit=5"

# Search companies
curl "http://127.0.0.1:8080/api/search?q=RELIANCE"

# Get market overview
curl http://127.0.0.1:8080/api/market/overview
```

## CORS Configuration

The API is configured with permissive CORS settings for development:
- Allows any origin (`*`)
- Allows any HTTP method
- Allows any headers
- Cache max-age: 3600 seconds

## Next Steps for Production

For production deployment, consider:
1. Implementing rate limiting
2. Adding authentication/authorization
3. Configuring specific CORS origins
4. Adding request validation middleware
5. Implementing caching for frequently accessed data
6. Adding OpenAPI/Swagger documentation
7. Setting up monitoring and metrics endpoints
