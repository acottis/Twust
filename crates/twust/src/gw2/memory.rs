//! Here we will interact with the mapped memory passed to us from gw2 using 
//! the rumble lib
use rumble::LinkedMem;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Identity{
    name: String,
    profession: u8,
    spec: u16,
    race: u8,
    pub map_id: u32,
    world_id: u32,
    team_color_id: u8,
    commander: bool,
    map: u32,
    fov: f32,
    uisz: u8,
}

pub fn parse_identity(memory: &LinkedMem) -> Identity {
    serde_json::from_str(&memory.identity())
        .expect("Could not parse identity from memory")
}

/// Gets the ip address from the context section of the mapped file
pub fn ip_address(memory: &LinkedMem) -> String {
    format!("{}.{}.{}.{}", 
        memory.context[4],
        memory.context[5],
        memory.context[6],
        memory.context[7]
    )
}