extern crate curl;
extern crate serde;
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate chrono;

extern crate select;
extern crate reqwest;
extern crate irc;

use std::env;
use std::io;
use irc::client::prelude::*;

mod simulator;
mod commands;

fn run_user_command(args: Vec<String>, nick: String) -> Option<String> {
    if args.len() < 1 {
        return None;
    }
    let base_cmd = &args[0];
    let mut response: Option<String> = None;
    if base_cmd == "price" {
        let svc = commands::stock_commands::StockValueCommand { symbol: args.get(1).unwrap_or(&String::from("")).to_owned(), };
        response = Some(svc.execute());
    }
    if base_cmd == "start" {
        let ic = commands::simulator_commands::InitCommand {
            nick: nick.to_owned(),
            registered: true,
        };
        response = Some(ic.execute());
    }
    if base_cmd == "buy" {
        let bc = commands::simulator_commands::BuyCommand {
            nick: nick.to_owned(),
            symbol: args.get(2).unwrap_or(&String::from("")).to_owned(),
            quantity: args.get(1).unwrap_or(&String::from("")).to_owned(),
        };
        response = Some(bc.execute());
    }
    if base_cmd == "sell" {
        let sc = commands::simulator_commands::SellCommand {
            nick: nick.to_owned(),
            symbol: args.get(2).unwrap_or(&String::from("")).to_owned(),
            quantity: args.get(1).unwrap_or(&String::from("")).to_owned(),
        };
        response = Some(sc.execute());
    }
    if base_cmd == "cash" {
        let cc = commands::simulator_commands::CashCommand { nick: nick.to_owned() };
        response = Some(cc.execute());
    }
    if base_cmd == "portfolio" || base_cmd == "pf" {
        let pc = commands::simulator_commands::PortfolioCommand { nick: nick.to_owned() };
        response = Some(pc.execute());
    }
    if base_cmd == "net" {
        response = Some("Very rich!".to_owned());
    }
    if base_cmd == "info" {
        let ic  = commands::stock_commands::InfoCommand { symbol: args.get(1).unwrap_or(&String::from("")).to_owned(), };
        response = Some(ic.execute());
    }
    if base_cmd == "news" {
        let nc  = commands::stock_commands::NewsCommand { symbol: args.get(1).unwrap_or(&String::from("")).to_owned(), };
        response = Some(nc.execute());
    }
    if base_cmd == "help" || base_cmd == "commands" {
        return Some("Commands: .price, .start, .buy <qty> <symbol>, .sell <qty> <symbol>,.cash, .portfolio, .net".to_owned());
    }
    return response;
}

fn run_irc() {
    let config = Config {
        nickname: Some("daytrader_bot".to_owned()),
        server: Some("irc.rizon.net".to_owned()),
        channels: Some(vec!["#test_daytrader".to_owned()]),
        ..Config::default()
    };

    //let config = Config::load("config/irc.toml").expect("failed to load irc config");

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).expect("failed to connect");
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        print!("{}", message);
        if let &Command::PRIVMSG(ref ch, ref msg) = &message.command {
            let mut args: Vec<String> = msg.split_whitespace().map(|s| s.to_string()).collect();
            let nick = message
                .source_nickname()
                .expect("no nick found")
                .to_string();
            let cmd_hint = args[0].remove(0).to_string();
            if cmd_hint == "." {
                if let Some(s) = run_user_command(args.clone(), nick.to_owned()) {
                    client.send_notice(&nick, &format!("{}", s)).unwrap();
                }
            }
            if cmd_hint == "@" {
                if let Some(s) = run_user_command(args.clone(), nick.to_owned()) {
                    client.send_privmsg(&ch, &format!("{}", s)).unwrap();
                }
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}

fn run_cli() {
    let nick = String::from("baggins");
    loop {
        let mut input = String::new();
        println!("\n>");
        io::stdin().read_line(&mut input).expect(
            "failed to read line",
        );

        input = input.trim().to_owned();
        let args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        if let Some(s) = run_user_command(args, nick.to_owned()) {
            println!("=> {}", s);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments: use either -irc or -cli");
    }
    
    if args.contains(&String::from("-cli")) {
        run_cli();
    }
    else if args.contains(&String::from("-irc")) {
        run_irc();
    }
}

