use std::collections::HashMap;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use simulator::stock_api;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub nick: String,
    pub cash: f64,
    pub shares: HashMap<String, i64>,
}

impl Player {
}

pub fn create_player(nick: String) -> Result<(), String> {
    if let Some(_p) = get_player(nick.to_owned()) {
        println!("Player already exists.. not creating");
        return Err("Player already exists".to_string());
    }

    let new_player = Player {
        nick: nick.to_owned(),
        cash: 100000.00,
        shares: HashMap::new(),
    };

    let mut players: Vec<Player> = read_players();
    players.push(new_player);
    write_players(players);

    return Ok(());
}

pub fn get_player(nick: String) -> Option<Player> {
    for player in read_players() {
        if player.nick == nick {
            return Some(player);
        }
    }

    return None;
}

fn write_players(players: Vec<Player>) {
    let json = serde_json::to_string(&players).unwrap();

    let mut file = File::create("players.json").expect("unable to create file");
    file.write_all(json.as_bytes()).expect(
        "unable to write data",
    );
}

pub fn read_players() -> Vec<Player> {
    let mut file = File::open("players.json").expect("players.json not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Unable to read_to_string",
    );
    contents = contents.trim().to_string();

    if contents.is_empty() {
        return Vec::new();
    }
    
    return serde_json::from_str(&contents).expect("failed to create player list");
}

pub fn buy_stock(nick: String, symbol: String, qty: i64) -> Result<(), String> {
    let symbol = symbol.to_uppercase();
    let mut players_out: Vec<Player> = Vec::new();
    for mut p in read_players().to_owned() {
        let mut p2 = p;
        if p2.nick == nick {
            let total = qty as f64 * stock_api::get_current_price(&symbol)?;
            if p2.cash < total {
                return Err(format!("{} has insufficient funds", p2.nick).to_owned());
            }
            p2.cash -= total;
            *p2.shares.entry(symbol.to_string()).or_insert(0) += qty;
        }
        players_out.push(p2);
    }
    write_players(players_out.to_owned());

    return Ok(());
}

pub fn sell_stock(nick: String, symbol: String, qty: i64) -> Result<(), String> {
    let symbol = symbol.to_uppercase();
    let mut players_out: Vec<Player> = Vec::new();
    for p in read_players().to_owned() {
        let mut updated = p;
        if updated.nick == nick {
            let shares_held = *updated.shares.entry(symbol.to_string()).or_insert(0);
            if qty > shares_held {
                return Err(format!("{} doesn't have enough shares of {}", updated.nick, symbol).to_owned());
            }

            let total = qty as f64 * stock_api::get_current_price(&symbol)?;
            updated.cash += total;
            *updated.shares.entry(symbol.to_string()).or_insert(0) -= qty;
            let shares_held = *updated.shares.entry(symbol.to_string()).or_insert(0);
            if shares_held == 0 {
                updated.shares.remove(&symbol.to_string());
            }
        }
        players_out.push(updated);
    }
    write_players(players_out.to_owned());
    return Ok(());
}
