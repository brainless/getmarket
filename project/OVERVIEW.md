We are creating a product that tracks listed companies, their stock, articles, PR, etc. All from publicly available data. Users can create their portfolio to manage what they track, get alerts.

We are going to start with stocks in India. Here are the details for each free daily trade data source for Indian stock exchanges, including precise URLs, data formats, and authentication requirements:

***

### 1. **NSE India (National Stock Exchange of India)**

- **URL(s):**
    - Official site: https://www.nseindia.com
    - Daily/Monthly historical archives: https://www.nseindia.com/resources/historical-reports-capital-market-daily-monthly-archives
    - Bhavcopy (end-of-day summary, widely used): https://www.nseindia.com/all-reports
- **Data Format:** CSV (bhavcopy files), ZIP (for bulk/downloads), and direct HTML tables for summary views.
- **Authentication:** No authentication is needed to download daily bhavcopy and summary data. For personalized trade verification (Nice Plus), registration is required, but market-wide daily data is fully open-access.[^1][^2][^3]
- **Access Frequency:** Daily (updated EOD, typically available T+1).

***

### 2. **BSE India (Bombay Stock Exchange)**

- **URL:**
    - Historical data: https://www.bseindia.com/market_data.html
    - Market statistics and reports are accessible via the BSE homepage: https://www.bseindia.com
- **Data Format:** CSV, XLS, and direct web tables. Most data is downloadable as daily, weekly, or monthly files.
- **Authentication:** Open access for most EOD and historical data. No registration needed for daily trade files.[^4]
- **Access Frequency:** Daily (EOD data usually published in the evening, IST).

***

### 3. **Kaggle (India Stock Market Datasets)**

- **URL:**
    - Sample dataset: https://www.kaggle.com/datasets/adritpal08/indian-stock-market-dataset
    - Alternative: https://www.kaggle.com/datasets/hk7797/stock-market-india
- **Data Format:** CSV (single or multiple large files with historical prices, trades, etc.), Zipped CSV for bulk datasets.
- **Authentication:** Free, but Kaggle account (Google/email) is required for download. Data is community-curated.[^5][^6]
- **Access Frequency:** Dataset update frequency varies; daily updates are common for actively maintained datasets.

***

### 4. **Yahoo Finance**

- **URL:**
    - Frontend: https://in.finance.yahoo.com
    - Data can be downloaded for any Indian symbol, e.g., NSE:RELIANCE or BSE:SENSEX.
    - API data available via Python libraries like yahoo_fin or yfinance.
- **Data Format:** CSV (manual download), JSON (API/library access), DataFrame (library output).
- **Authentication:** Web download requires no login; for API, no authentication is needed for most queries via the yfinance or yahoo_fin Python packages.[^7]
- **Access Frequency:** Daily (historical and live EOD data; sometimes with short delay).

***

### 5. **Upstox Trading \& Market Data API**

- **URL:** https://upstox.com/developer/api-documentation/authentication/
- **Data Format:** JSON (API output, easily converted to CSV/DataFrame).
- **Authentication:** Requires free developer registration (API key generation); authentication is via OAuth 2.0. All logins require use of Upstox-compliant authorization flow.[^8][^9]
- **Access Frequency:** Real-time and historical data (APIs accessible any time; registration is fast).

***

### 6. **Moneycontrol**

- **URL:** https://www.moneycontrol.com
    - Individual company data (e.g., https://www.moneycontrol.com/technical-analysis/datapatternsindia/DPI01/daily)
- **Data Format:** Web tables (copy-paste or web scraping to CSV/XLS), summary downloadable as PDFs for some sections.
- **Authentication:** No login needed for most summary and technical data. Registered users may access portfolio/alert features, but daily trade data itself is open.[^10][^11]
- **Access Frequency:** Updated daily, EOD.

***

### 7. **Rediff Money**

- **URL:** https://portfolio.rediff.com/portfolio
- **Data Format:** Web tables, CSV/PDF (for exporting your own portfolio). Data columns typically include symbol, quantity, buy price, date, etc.
- **Authentication:** No login to view current market data; export/download of personalized portfolio data may require free registration.[^12][^13]
- **Access Frequency:** Daily updates for public market data and indicators.

***

**Note:**

- **Official exchange sites (NSE, BSE)** always provide the most reliable and complete data.
- **Kaggle/Yahoo Finance** are best for easy-downloading and scripting, suitable for historians and developers.
- **APIs (Upstox)** are ideal for automation but do require a one-time registration (free).
- **Aggregator sites (Moneycontrol, Rediff Money)** suit quick overviews and manual analysis.


## References

[^1]: https://www.nseindia.com/invest/first-time-investor-trade-verification
[^2]: https://www.nseindia.com/all-reports
[^3]: https://www.nseindia.com/resources/historical-reports-capital-market-daily-monthly-archives
[^4]: https://www.bseindia.com/market_data.html
[^5]: https://www.kaggle.com/datasets/adritpal08/indian-stock-market-dataset
[^6]: https://www.kaggle.com/datasets/hk7797/stock-market-india
[^7]: https://algotrading101.com/learn/yahoo-finance-api-guide/
[^8]: https://community.upstox.com/t/how-to-do-upstox-api-authentication/3422
[^9]: https://upstox.com/developer/api-documentation/authentication/
[^10]: https://www.moneycontrol.com/technical-analysis/datapatternsindia/DPI01/daily
[^11]: https://www.moneycontrol.com
[^12]: https://portfolio.rediff.com/portfolio
[^13]: https://smartincomeidea.com/rediff-money-review/
[^14]: https://www.nseindia.com/market-data/real-time-data-subscription
[^15]: https://www.nseindia.com
[^16]: https://www.nseindia.com/market-data/eod-historical-data-subscription
[^17]: https://globaldatafeeds.in/global-datafeeds-nsebse-mcx-authorized-data-vendor/
[^18]: https://www.nseindia.com/reports/fii-dii
[^19]: https://stackoverflow.com/questions/28822780/fetching-1-minute-bars-from-yahoo-finance
[^20]: https://www.nseindia.com/market-data/live-equity-market
