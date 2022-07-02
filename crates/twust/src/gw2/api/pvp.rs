//! This module handles all the [PVP Endpoints](https://wiki.guildwars2.com/wiki/API:2/pvp)

/// https://wiki.guildwars2.com/wiki/API:2/pvp/standings
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Standings{
    current: Current,
    best: Best,
    season_id: String,
}

/// https://wiki.guildwars2.com/wiki/API:2/pvp/standings
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Current{
    total_points: u16,
    division: u8,
    tier: u8,
    points: u8,
    repeats: u8,
    rating: u16,
}

/// https://wiki.guildwars2.com/wiki/API:2/pvp/standings
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Best{
    total_points: u16,
    division: u8,
    tier: u8,
    points: u8,
    repeats: u8,
}