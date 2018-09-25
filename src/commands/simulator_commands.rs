use simulator::records;
use simulator::stock_api;

pub struct InitCommand {
    pub nick: String,
    pub registered: bool,
}

impl InitCommand {
    pub fn execute(&self) -> String {
        match records::create_player(self.nick.to_string()) {
            Ok(()) => return format!("Thanks for starting the simulator, {}! Type .help for more information.", self.nick),
            Err(err) => return format!("Could not create player '{}': {}", self.nick, err),
        }
    }
}

pub struct BuyCommand {
    pub nick: String,
    pub symbol: String,
    pub quantity: String,
}

impl BuyCommand {
    pub fn execute(&self) -> String {
        let quantity = self.quantity.parse();
        match quantity {
            Ok(_n) => {},
            Err(_e) => return format!("{} is not a valid quantity.", self.quantity),
        }

        let quantity = quantity.unwrap();
        match records::buy_stock(
            self.nick.to_string(),
            self.symbol.to_string(),
            quantity,
        ) {
            Ok(()) => return format!("{} bought {} shares of {} for a total of ${:.*}", 
                //self.nick, self.quantity, self.symbol, 2, self.quantity.unwrap() as f64 *stock_api::get_current_price(&self.symbol).unwrap()),
                self.nick, quantity, self.symbol, 2, quantity as f64 *stock_api::get_current_price(&self.symbol).unwrap()),
            Err(err) => return err,
        }
    }
}

pub struct SellCommand {
    pub nick: String,
    pub symbol: String,
    pub quantity: String,
}

impl SellCommand {
    pub fn execute(&self) -> String {
        let quantity = self.quantity.parse();
        match quantity {
            Ok(_n) => {},
            Err(_e) => return format!("{} is not a valid quantity.", self.quantity),
        }

        let quantity = quantity.unwrap();
        match records::sell_stock(
            self.nick.to_string(),
            self.symbol.to_string(),
            quantity,
        ) {
            Ok(()) => return format!("{} sold {} shares of {} for a total of ${:.*}", 
                self.nick, quantity, self.symbol, 2, quantity as f64 *stock_api::get_current_price(&self.symbol).unwrap()),
            Err(err) => return err,
        }
    }
}

pub struct PortfolioCommand {
    pub nick: String,
}

impl PortfolioCommand {
    pub fn execute(&self) -> String {
        if let Some(p) = records::get_player(self.nick.to_owned()) {
            let mut value: f64 = 0.0;
            for (symbol, qty) in &p.shares {
                value += *qty as f64 * stock_api::get_current_price(&symbol).unwrap();
            }
            return format!("Value: ${:.*} ** Portfolio: {:?}", 2, value+p.cash, p.shares);
        } else {
            return "No portfolio found. Use the .start command to play!".to_string();
        }
    }
}

pub struct CashCommand {
    pub nick: String,
}

impl CashCommand {
    pub fn execute(&self) -> String {
        let p = records::get_player(self.nick.to_owned()).expect("Failed to get player");
        return format!("{} has ${:.2} in cash.", self.nick.to_string(), p.cash);
    }
}
