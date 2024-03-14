# Helldivers Rust API Wrapper

A Rust wrapper for the unofficial Helldivers API. This library provides an easy way to interact with the Helldivers API 
and retrieve information about wars, planets, factions, and sectors.

Please do note, the Helldivers API is unofficial and may be subject to change at any time. 
More functions will be added as the API is reverse-engineered.

This library is not affiliated with Arrowhead Game Studios or Sony Interactive Entertainment.

# Installation

Run the following Cargo command in your project directory:
```shell
cargo add helldivers_rs
```

Or add the following line to your Cargo.toml:
```toml
[dependencies]
helldivers_rs = "0.3"
```

# API Reference

The library provides the following functions:

    get_status(war_id: i64, language: &str) -> Result<Status, reqwest::Error>: Get the current status of a war.
    get_war_info(war_id: i64) -> Result<WarInfo, reqwest::Error>: Get information about a specific war.
    get_planet_name(id: i64) -> String: Get the name of a planet by its ID.
    get_faction_name(id: i64) -> String: Get the name of a faction by its ID.
    get_sector_name(id: i64) -> String: Get the name of a sector by its ID.

For more details about the structs and their fields, please refer to the source code documentation.

# Example 

```rust
use helldivers_rs;
use tokio;

#[tokio::main]
async fn main() {
    // Get the current status of a war
    let war_id = 801; // The war ID for the current war
    let status = helldivers_rs::get_status(war_id, "en-US").await.unwrap();
    println!("Current Message: {}", status.global_events[0].message);

    // Get information about a specific war
    let war_info = helldivers_rs::get_war_info(war_id).await.unwrap();
    println!("War Start Date: {}", war_info.start_date);

    // Get the name of a planet by ID
    let planet_id = 0;
    let planet_name = helldivers_rs::get_planet_name(planet_id);
    println!("Planet Name: {}", planet_name);

    // Get the name of a faction by ID
    let faction_id = 1;
    let faction_name = helldivers_rs::get_faction_name(faction_id);
    println!("Faction Name: {}", faction_name);

    // Get the name of a sector by ID
    let sector_id = 0;
    let sector_name = helldivers_rs::get_sector_name(sector_id);
    println!("Sector Name: {}", sector_name);
}
```

# Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

# License

This project is licensed under the MIT License.