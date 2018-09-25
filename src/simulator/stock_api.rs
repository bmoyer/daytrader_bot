use curl::easy::Easy;
use chrono::NaiveDateTime;

use serde_json::{Value, Map};

use std;
use serde_json;

pub struct CompanyData {
    pub name: String,
    pub industry: String,
    pub description: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompanyNews {
    pub datetime: String,
    pub headline: String,
    pub source: String,
    pub url: String,
    pub summary: String,
    pub related: String,
    pub image: String,
}

#[derive(Debug)]
struct Price {
    date: NaiveDateTime,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

pub fn get_last_news(symbol: &str) -> Result<CompanyNews, String> {
    let url = format!("https://api.iextrading.com/1.0/stock/{}/company", symbol);
    let json = get_raw_json(&url);

    //let v: Vec<Map<String, Value>>= serde_json::from_str(&json).expect("news list was empty");
    let v: Vec<CompanyNews>= serde_json::from_str(&json).expect("news list was empty");
    return Ok(v[0].clone());

    /*
    if v.get("companyName").is_none() {
        return Err("Did not find company data".to_owned());
    }

    let name = v.get("companyName").unwrap().to_string();
    let industry = v.get("industry").unwrap().to_string();
    let description = v.get("description").unwrap().to_string();

    */
    //return Ok(CompanyNews { headline: headline, summary: summary, url: url });
}

pub fn get_company_data(symbol: &str) -> Result<CompanyData, String> {
    let url = format!("https://api.iextrading.com/1.0/stock/{}/company", symbol);
    let json = get_raw_json(&url);

    let v: Map<String, Value> = serde_json::from_str(&json).unwrap();

    if v.get("companyName").is_none() {
        return Err("Did not find company data".to_owned());
    }

    let name = v.get("companyName").unwrap().to_string();
    let industry = v.get("industry").unwrap().to_string();
    let description = v.get("description").unwrap().to_string();

    return Ok(CompanyData { name: name,
        industry: industry, description: description });
}

pub fn get_raw_json(url: &str) -> String {
    let mut html: String = String::new();
    {
        let mut easy = Easy::new();
        easy.url(url).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                html.push_str(std::str::from_utf8(data).unwrap());
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }
    return html;
}

pub fn get_current_price(symbol: &str) -> Result<f64, String> {
    let url = format!("https://api.iextrading.com/1.0/stock/{}/price", symbol);
    let json = get_raw_json(&url);

    match json.parse::<f64>() {
        Ok(n) => return Ok(n),
        Err(e) => return Err(format!("Failed to get price for {}: {}", symbol, e.to_string())),
    }
}

/*
pub fn get_daily_change_iex(symbol: &str) -> Result<f64, String> {
    let url = format!(
        "https://api.iextrading.com/1.0/stock/{}/chart/dynamic",
        symbol
    );
    let json = get_raw_json(&url);

    let v: Map<String, Value> = serde_json::from_str(&json).unwrap();
    let series = v.get("data");

    // call function taking (symbol, date) which gives price struct (contains open, close, volume,
    // high, low, etc. call for today and previous day, take differnce
}
*/

pub fn get_daily_change(symbol: &str) -> Result<f64, String> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey=S7O9IAZ5R601VAKK",
        symbol
    );
    let json = get_raw_json(&url);

    let v: Map<String, Value> = serde_json::from_str(&json).unwrap();
    let series = v.get("Time Series (Daily)");

    //let mut oo: &Map<String, Value> = &Map::new();
    let oo;
    if let Some(_v) = series {
        oo = series.unwrap().as_object().unwrap();
    } else {
        let err = format!("Failed to get change for: {}", symbol.to_uppercase());
        return Err(err);
    }

    let mut price_history: Vec<Price> = Vec::new();
    for y in oo.iter() {
        let z: &Map<String, Value> = y.1.as_object().unwrap();
        let price_date = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", y.0), "%Y-%m-%d %T")
            .unwrap();
        price_history.push(Price {
            date: price_date,
            open: str::replace(&z["1. open"].to_string(), "\"", "")
                .parse()
                .unwrap(),
            high: str::replace(&z["2. high"].to_string(), "\"", "")
                .parse()
                .unwrap(),
            low: str::replace(&z["3. low"].to_string(), "\"", "")
                .parse()
                .unwrap(),
            close: str::replace(&z["4. close"].to_string(), "\"", "")
                .parse()
                .unwrap(),
            volume: str::replace(&z["5. volume"].to_string(), "\"", "")
                .parse()
                .unwrap(),
        });
    }
    let current = &price_history[&price_history.len() - 1].close;
    let last = &price_history[&price_history.len() - 2].close;

    let percent_change = ((current - last) / last) * 100.0;
    return Ok(percent_change);
}
