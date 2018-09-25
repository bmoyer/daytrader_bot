use simulator::stock_api;

pub struct StockValueCommand {
    pub symbol: String,
}

impl StockValueCommand {
    pub fn execute(&self) -> String {
        let change = get_price_cmd(&self.symbol);
        return change;
    }
}

pub struct InfoCommand {
    pub symbol: String,
}

impl InfoCommand {
    pub fn execute(&self) -> String {
        let info = stock_api::get_company_data(&self.symbol);
        match info {
            Err(e) => return format!("Error: {}", e),
            Ok(i) => return format!("{} [{}] -- {}", i.name, i.industry, i.description),
        }
        
    }
}

pub struct NewsCommand {
    pub symbol: String,
}

impl NewsCommand {
    pub fn execute(&self) -> String {
        let info = stock_api::get_last_news(&self.symbol);
        match info {
            Err(e) => return format!("Error: {}", e),
            Ok(n) => return format!("[{}] {} {} ({})", n.datetime, n.headline, n.summary, n.url),
        }
        
    }
}

fn format_percent_change(percent: f64) -> String {
    if percent > 0.0 {
        return format!("\x033+{:.*}%\x03", 2, percent);
    } else if percent < 0.0 {
        return format!("\x034{:.*}%\x03", 2, percent);
    } else {
        return format!("{:.*}%", 2, percent);
    }
}

fn get_price_cmd(symbol: &str) -> String {
    let change = stock_api::get_daily_change(symbol);
    match change {
        Err(e) => return format!("Error: {}", e),
        Ok(c) => {
            return format!(
                "*** [ STOCK PRICE ]: {} ${}: {} since last close",
                symbol.to_uppercase(),
                stock_api::get_current_price(symbol).unwrap(),
                format_percent_change(c)
            )
        }
    }
}

