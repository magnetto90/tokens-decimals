import requests
import time

# Coin IDs
coin_ids = ["usd-coin", "tether", "dai", "true-usd", "binance-usd", "binancecoin",
            "staked-ether", "usdd", "frax", "paxos-standard", "matic-network", "the-open-network",
            "shiba-inu", "wrapped-bitcoin", "leo-token", "chainlink", "uniswap", "okb", "lido-dao",
            "arbitrum", "crypto-com-chain", "maker", "rocket-pool-eth", "aave", "pepe"]

# Sort the coin IDs alphabetically
coin_ids.sort()

# Initialize the Markdown table
markdown_table = "# EVM Tokens Decimals List\n"
markdown_table += "| Name | Decimals (Ethereum) | Decimals (Other Platforms) |\n"
markdown_table += "| ---- | ------------------- | ------------------------- |\n"

# Delay between API calls (in seconds)
delay_seconds = 4

# Make a request to the CoinGecko API for each coin
for coin_id in coin_ids:
    url = f"https://api.coingecko.com/api/v3/coins/{coin_id}"
    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()

        # Extract coin details
        name = data["name"]
        ethereum_decimals = data["detail_platforms"]["ethereum"]["decimal_place"]

        # Initialize the list of decimals on other platforms
        other_platform_decimals = []

        # Compare decimals on other platforms with Ethereum
        for platform, details in data["detail_platforms"].items():
            if platform != "ethereum" and platform != "" and details["decimal_place"] != ethereum_decimals and details["decimal_place"] != "None":
                other_platform_decimals.append(
                    f"{platform}: **{details['decimal_place']}**"
                )

        # Convert the list of decimals to a multiline string
        other_platform_decimals_str = "\n|||".join(
            other_platform_decimals) if other_platform_decimals else "N/A"

        # Add the data to the table if the coin has decimals on other platforms
        if other_platform_decimals:
            markdown_table += f"| {name} | {ethereum_decimals} | {other_platform_decimals_str} |\n"

        # Add a delay to avoid hitting the API rate limit
        time.sleep(delay_seconds)
    else:
        print(
            f"Error calling the API for {coin_id}. Status code: {response.status_code}")

# Save the table to a file named "readme.md"
with open("README.md", "w") as readme_file:
    readme_file.write(markdown_table)

print("File 'readme.md' generated successfully.")
