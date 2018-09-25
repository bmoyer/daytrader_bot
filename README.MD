# DayTraderBot
This program is a simple stock market simulator bot. There are two basic modes of operation:
- Run DayTraderBot as a bot on an IRC network
- Interact with DayTraderBot directly through a command-line interface (mostly for debugging)

## Running the simulator
```
git clone https://github.com/bmoyer/daytrader_bot
cd daytrader_bot
cargo build
./target/debug/ircticker -irc (for irc mode)
./target/debug/ircticker -cli (for cli mode)
```

## Example usage
As with most IRC bots, commands are preceded with @ to indicate public output and . to indicate private output. No command prefix is used in CLI mode.
```
[19:46] <@jordan_belfort> .start
[19:46] -daytrader_bot- Thanks for starting the simulator, jordan_belfort! Type .help for more information.
[19:46] <@jordan_belfort> .portfolio
[19:46] -daytrader_bot- Value: $100000.00 ** Portfolio: {}
[19:47] <@jordan_belfort> .buy 100 INTC
[19:47] -daytrader_bot- jordan_belfort bought 100 shares of INTC for a total of $4691.00
[19:47] <@jordan_belfort> .price AAPL
[19:47] -daytrader_bot- *** [ STOCK PRICE ]: AAPL $220.79: +1.44% since last close
[19:47] <@jordan_belfort> .buy 200 aapl
[19:47] -daytrader_bot- jordan_belfort bought 200 shares of aapl for a total of $44158.00
[19:47] <@jordan_belfort> .portfolio
[19:47] -daytrader_bot- Value: $100000.00 ** Portfolio: {"AAPL": 200, "INTC": 100}
[19:48] <@jordan_belfort> .help
[19:48] -daytrader_bot- Commands: .price, .start, .buy <qty> <symbol>, .sell <qty> <symbol>,.cash, .portfolio, .net
```

## TODO ##
This is my Hello World for the Rust programming language, so this project will probably not get many updates. But a few outstanding items I'd like to complete:

- Switch .price command to use AlphaVantage API rather than IEX API, as it allows a higher request rate
- Allow the bot to use IRC config files compatible with the rust IRC crate
- Allow AlphaVantage API key to be set with a config file
