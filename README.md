# Helldive_rs 2 Rust API Wrapper
![GitHub Release](https://img.shields.io/github/v/release/LordOfPolls/helldive_rs?label=GitHub%20Release)
![Crates.io Version](https://img.shields.io/crates/v/helldive_rs?label=crates.io%20Version)
![Crates.io Total Downloads](https://img.shields.io/crates/d/helldive_rs)
![Crates.io License](https://img.shields.io/crates/l/helldive_rs)


A Rust wrapper for the unofficial Helldivers API. This library provides an easy way to interact with the Helldivers API 
and retrieve information about wars, planets, factions, and sectors.

Please do note, the Helldivers API is unofficial and may be subject to change at any time. 
More functions will be added as the API is reverse-engineered.

This library is not affiliated with Arrowhead Game Studios or Sony Interactive Entertainment. 
You are strongly encouraged to use the Helldivers API responsibly 
and to take care not to overload the servers... _more than they already are._

# Installation

Run the following Cargo command in your project directory:
```shell
cargo add helldive_rs 
```

Or add the following line to your Cargo.toml:
```toml
[dependencies]
helldive_rs  = "0.5"
```

# API Reference

The library provides the following functions:

    get_status(war_id: i64, language: &str) -> Result<Status, HelldiversError>: Get the current status of a war.
    get_war_info(war_id: i64) -> Result<WarInfo, HelldiversError>: Get information about a specific war.
    get_war_time(war_id: i64) -> Result<WarTime, HelldiversError>: Get the current time of a war.
    get_planet_name(id: i64) -> Option<String>: Get the name of a planet by its ID.
    get_faction_name(id: i64) -> Option<String>: Get the name of a faction by its ID.
    get_sector_name(id: i64) -> Option<String>: Get the name of a sector by its ID.
    // utils
    get_total_player_count(status: &Status) -> i64: Get the total number of players in a war.
    get_top_planets_by_player_count(status: &Status, count: usize) -> Vec<(&PlanetStatus, i64)>: Get the top planets by player count.
    get_faction_distribution(status: &Status) -> Hashmap<i64, i64>: Get the distribution of factions.


For more details about the structs and their fields, please refer to the source code documentation.

# Example 

```rust
use helldive_rs;
use helldive_rs::Language;
use tokio;

#[tokio::main]
async fn main() {
    // Get the current status of a war
    let war_id = 801; // The war ID for the current war
    let status = helldive_rs ::get_status(war_id, Language::English).await.unwrap();
    println!("Current Message: {}", status.global_events[0].message);

    // Get information about a specific war
    let war_info = helldive_rs ::get_war_info(war_id).await.unwrap();
    println!("War Start Date: {}", war_info.start_date);

    // Get the name of a planet by ID
    let planet_id = 0;
    let planet_name = helldive_rs ::get_planet_name(planet_id);
    println!("Planet Name: {}", planet_name);

    // Get the name of a faction by ID
    let faction_id = 1;
    let faction_name = helldive_rs ::get_faction_name(faction_id);
    println!("Faction Name: {}", faction_name);

    // Get the name of a sector by ID
    let sector_id = 0;
    let sector_name = helldive_rs ::get_sector_name(sector_id);
    println!("Sector Name: {}", sector_name);
}
```

# Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

# License

This project is licensed under the MIT License.