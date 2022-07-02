/// https://wiki.guildwars2.com/wiki/API:2/maps
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Map{
    id: u32,
    name: String,
    min_level: u8,
    max_level: u8,
    default_floor: u8,
    #[serde(rename = "type")]
    map_type: String,
    floors: Vec<u16>,
    region_id: u16,
    region_name: String,
    map_rect: [[i32; 2]; 2],
    continent_rect: [[i32; 2]; 2],
}

/// https://wiki.guildwars2.com/wiki/API:2/maps
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Maps(u32);