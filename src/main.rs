struct Token {
    name: String,
    symbol: String,
    total_supply: u64,
    price: f64,
}

fn print_token_info(token: &Token) {
    println!("Token Name: {}", token.name);
    println!("Token Symbol: {}", token.symbol);
    println!("Total Supply: {}", token.total_supply);
    println!("Current Price: ${:.2}", token.price); // Display price with 2 decimal places
}

pub fn main() {
    let my_favorite_token = Token {
        name: String::from("Ethereum"),
        symbol: String::from("ETH"),
        total_supply: 120_230_000, // Approximate current supply as of Jul 2024
        price: 3266.52,        // Example price in USD
    };

    // Call the print_token_info function
    print_token_info(&my_favorite_token);
}