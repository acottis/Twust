//! We get the map coords from the mumblelink integration enabled by the gw2
//! devs by reading Memory Mapped files that we create for gw2 to write to
//! Windows API - https://microsoft.github.io/windows-docs-rs/doc/windows
//! Mumble - https://wiki.mumble.info/wiki/Link#Linking_a_game_to_Mumble

use windows::Win32::System::Memory::{MapViewOfFile, CreateFileMappingW};

#[derive(Debug)]
pub enum Error{
    CreateFileMappingW(windows::core::Error)
}

type Result<T> = std::result::Result<T, Error>;

pub struct Link {
    handle: windows::Win32::Foundation::HANDLE,
    ptr: *const LinkedMem,
}

impl Link {
    
    /// Creates the memorymapped file and returns us the file handle and 
    /// pointer to the memory location of the size of our [LinkedMem] struct
    pub fn new() -> Result<Self> {
        
        // Name of memory mapped file that mumle is looking for null termed
        const FILE_MAPPING_NAME_UTF8: &'static str = "MumbleLink\0";
        
        // Convert name to utf16 
        let file_mapping_name: Vec<u16> 
            = FILE_MAPPING_NAME_UTF8.encode_utf16().collect();
        
        // Calculate size of [LinkedMem] so we can use to allocate enough space
        let linked_mem_size: usize = core::mem::size_of::<LinkedMem>();
        
        // Create the mapped file and get handle
        let handle = unsafe { 
            CreateFileMappingW(
                windows::Win32::Foundation::INVALID_HANDLE_VALUE,
                core::ptr::null(),
                windows::Win32::System::Memory::PAGE_READWRITE,
                0,
                linked_mem_size.try_into().unwrap(),
                windows::core::PCWSTR(file_mapping_name.as_ptr())
            )
        };
        let handle = match handle {
            Ok(handle) => handle,
            Err(e)=> {
                    return Err(Error::CreateFileMappingW(e))
            }
        };
        // Get the ptr to the LinkedMem so we can read from it
        let file_ptr = unsafe { 
            MapViewOfFile(
                handle,
                windows::Win32::System::Memory::FILE_MAP_ALL_ACCESS,
                0,
                0,
                linked_mem_size,
            )
        };
        
        Ok(Self{
            handle,
            ptr: file_ptr as *const LinkedMem,
        })                
    }

    // Gets the most up to date [LinkedMem] or returns None
    pub fn update(&self) -> Option<LinkedMem>{
        let data: LinkedMem = unsafe {
            core::ptr::read(self.ptr)
        };     
        if data.ui_version == 0{
            return None
        }
        return Some(data)
    }
}

/// https://wiki.mumble.info/wiki/Link#Linking_a_game_to_Mumble
#[derive(Debug)]
#[repr(C)]
pub struct LinkedMem{
    ui_version: u32,
    ui_tick: u32,
    
    // Position is map specifc, the center of a map is 0,0,0
    f_avatar_position: [f32; 3], // X, Z, Y (X is EastWest, Z is UP, Y is NorthSouth)
    f_avatar_front: [f32; 3],
    f_avatar_top: [f32; 3],

    name: [u16; 256],

    f_camera_position: [f32; 3],
    f_camera_front: [f32; 3],
    f_camera_top: [f32; 3],

    identity: [u16; 256],
    context_len: u32,
    pub context: [u8; 256],
    description: [u16; 2048],
}

impl LinkedMem{
    /// Get the application name set by developer
    pub fn name(&self) -> String {
        String::from_utf16_lossy(&self.name)
            .trim_end_matches("\0")
            .to_string()
    }
    // Identifier which uniquely identifies a certain player in a context (e.g. the ingame name).
    pub fn identity(&self) -> String {
        String::from_utf16_lossy(&self.identity)
            .trim_end_matches("\0")
            .to_string()
    }
    // Context should be equal for players which should be able to hear each other positional and
	// differ for those who shouldn't (e.g. it could contain the server+port and team)
    pub fn context(&self) -> &[u8] {
        &self.context[..self.context_len as usize]
    }
    pub fn description(&self) -> String {
        String::from_utf16_lossy(&self.description)
            .trim_end_matches("\0")
            .to_string()
    }
    pub fn ui_tick(&self) -> u32{
        self.ui_tick
    }
    pub fn avatar_position(&self) -> (f32, f32, f32){
        (
            self.f_avatar_position[0], 
            self.f_avatar_position[2], 
            self.f_avatar_position[1]
        )
    }
}

impl std::fmt::Display for LinkedMem{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) 
        -> std::result::Result<(), std::fmt::Error> 
    { 
        let name = self.name();
        let identity = self.identity();
        let context = self.context();
        let ava_pos_x = self.f_avatar_position[0];
        let ava_pos_z = self.f_avatar_position[1];
        let ava_pos_y = self.f_avatar_position[2];
        let ui_tick = self.ui_tick;
        write!(f, "---Linked Memory---\nApp: {name}\nUI Tick: {ui_tick}\nIdentity: {identity}\nContext: {context:?}\nPosition: x {ava_pos_x}, y {ava_pos_y}, z {ava_pos_z}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Tests we can create the memory mapped file and get its ptr
    #[test]
    fn init_link() {
        let link = Link::new().expect("Setting up link failed");
    }

    /// Test that the memory mapped file has been written to by the application
    #[test]
    fn memory_updated(){
        let link = Link::new().expect("Setting up link failed");
        link.update().expect("Memory mapped file is empty: Is the application targeting it open?");
    }
}

/// A more readable way of interacting with the coordination
#[derive(Debug)]
pub struct AvatarPosition{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AvatarPosition{

    /// Pass in memory and get the avatar coordinates
    pub fn get(linked_mem: &LinkedMem) -> Self{
        Self{
            x: linked_mem.f_avatar_position[0], 
            y: linked_mem.f_avatar_position[2], 
            z: linked_mem.f_avatar_position[1]
        }
    }
}