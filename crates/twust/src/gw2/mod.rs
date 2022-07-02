//! Here we deal with all things [GW2 API](https://wiki.guildwars2.com/wiki/API:Main)

/// The pvp API endpoints
mod api;
mod memory;
mod error;

pub use api::Api;
