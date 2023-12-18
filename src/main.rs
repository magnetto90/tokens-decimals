use reqwest;
use serde_json::Value;
use std::{fs::File, io::Write, thread, time::Duration};

#[tokio::main]
async fn main() {
    // Coin IDs
    let mut coin_ids = vec![
        "usd-coin", "tether", "dai", "true-usd", "binance-usd", "binancecoin",
        "staked-ether", "usdd", "frax", "paxos-standard", "matic-network", "the-open-network",
        "shiba-inu", "wrapped-bitcoin", "leo-token", "chainlink", "uniswap", "okb", "lido-dao",
        "arbitrum", "crypto-com-chain", "maker", "rocket-pool-eth", "aave", "pepe",
    ];

    // Sort the coin IDs alphabetically
    coin_ids.sort();

    // Initialize the Markdown table
    let mut markdown_table = String::from("# EVM Tokens Decimals List\n| Name | Original Decimals | Decimals (Other Platforms) |\n| ---- | ------------------- | ------------------------- |\n");

    // Delay between API calls (in seconds)
    let delay_seconds = Duration::from_secs(10);

    // Make a request to the CoinGecko API for each coin
    for coin_id in coin_ids {
        let url = format!("https://api.coingecko.com/api/v3/coins/{}", coin_id);
        let response = reqwest::get(&url).await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: Value = resp.json().await.unwrap();

                    // Extract coin details
                    let name = data["name"].as_str().unwrap();
                    let ethereum_decimals = data["detail_platforms"]["ethereum"]["decimal_place"].as_i64().unwrap();

                    // Initialize the list of decimals on other platforms
                    let mut other_platform_decimals = vec![];

                    // Compare decimals on other platforms with Ethereum
                    if let Some(platforms) = data["detail_platforms"].as_object() {
                        for (platform, details) in platforms {
                            if platform != "ethereum" && details["decimal_place"].as_i64().is_some() && details["decimal_place"].as_i64().unwrap() != ethereum_decimals {
                                other_platform_decimals.push(format!("{}: **{}**", platform, details["decimal_place"].as_i64().unwrap()));
                            }
                        }
                    }

                    // Convert the list of decimals to a multiline string
                    let other_platform_decimals_str = if !other_platform_decimals.is_empty() {
                        other_platform_decimals.join("\n|||")
                    } else {
                        String::from("N/A")
                    };

                    // Add the data to the table if the coin has decimals on other platforms
                    if !other_platform_decimals.is_empty() {
                        markdown_table.push_str(&format!("| {} | {} | {} |\n", name, ethereum_decimals, other_platform_decimals_str));
                    }
                } else {
                    println!("Error calling the API for {}. Status code: {}", coin_id, resp.status());
                }
            }
            Err(e) => {
                println!("Error calling the API for {}: {:?}", coin_id, e);
            }
        }

        // Add a delay to avoid hitting the API rate limit
        thread::sleep(delay_seconds);
    }

    // Save the table to a file named "readme.md"
    let mut readme_file = File::create("README.md").unwrap();
    readme_file.write_all(markdown_table.as_bytes()).unwrap();

    println!("File 'readme.md' generated successfully.");
}
