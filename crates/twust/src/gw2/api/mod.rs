mod pvp;
mod maps;

use super::error::{Result, Error};

/// [Tokeninfo](https://wiki.guildwars2.com/wiki/API:2/tokeninfo)
#[derive(Debug, serde::Deserialize, Default)]
struct TokenInfo{
    id: String,
    name: String,
    permissions: Vec<String>,
}

/// Store information required to make API requests here and information on the token
#[derive(Debug)]
pub struct Api{
    key: String,
    token_info: TokenInfo
}

impl Api{

    const BASE_ENDPOINT: &'static str = "https://api.guildwars2.com/v2";

    /// Create new instance of Api with a key, return error is api key is wrong
    pub fn new(api_key: &str) -> Result<Self> {

        let api = Self {
            key: api_key.to_owned(),
            token_info: Default::default(),
        };

        let res = api.get("tokeninfo");

        let token_info: TokenInfo = res.unwrap().into_json()
            .map_err(Error::Deserialize)?;

        Ok(Self {
            key: api_key.to_owned(),
            token_info
        })
    }
    
    // Returns data from pvp/standings
    pub fn pvp_standings(&self) -> Result<Vec<pvp::Standings>> {

        let res = self.get("pvp/standings");

        res.unwrap()
            .into_json::<Vec<pvp::Standings>>()
            .map_err(Error::Deserialize)
    }

    /// Returns all maps
    fn maps(&self) -> Result<Vec<maps::Maps>> {
        
        let res = self.get("maps");

        res.unwrap()
            .into_json::<Vec<maps::Maps>>()
            .map_err(Error::Deserialize)
    }

    // Returns information on a specfied map
    fn map(&self, map_id: u32) -> Result<maps::Map> {
        let res = self.get(&format!("maps/{map_id}"));

        res.unwrap()
            .into_json::<maps::Map>()
            .map_err(Error::Deserialize)
    }

    /// Gets my map and coords
    pub fn where_am_i(&self, link: rumble::Link) -> Result<()>{
        match link.update(){
            Some(data) => {
                let identity = crate::gw2::memory::parse_identity(&data);
                println!("{:#?}", identity);
                let map = self.map(identity.map_id).unwrap();
                println!("{:#?}", map);

                let player = rumble::AvatarPosition::get(&data);
                println!("{:#?}", player);
                Ok(())
            }
            None => Err(Error::NoDataInMemory)
        }
    }
    
    /// Helper function for ureq get
    fn get(&self, endpoint: &str) -> Result<ureq::Response> {

        let url = &format!("{}/{}", Self::BASE_ENDPOINT, endpoint);
        let req = ureq::get(url)
            .set("Authorization", &format!("Bearer {}", self.key));    
        let res = req.call();
    
        match res {
            Ok(res) => {   
                Ok(res)
            }
            Err(ureq::Error::Status(code, response)) => {
                let text = response.status_text();
                let url = response.get_url();
                Err(Error::Http(format!("URL: {url} | Status Code: {code} | {text:?}")))
            }
            Err(e) => {
                Err(Error::UnknownHttp(e.to_string()))
            }
        }
    }
}
