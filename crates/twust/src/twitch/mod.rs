//! Deals with twitch API, right now just oauth2
/// https://id.twitch.tv/oauth2/token

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    EnvVarMissing(&'static str),
    CantGetRefreshToken(ureq::Error)
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct Token {
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    scope: Vec<String>,
    token_type: String,
}

impl Token {
    // URL for oauth related requests
    const URL: &'static str = "https://id.twitch.tv/oauth2/token";

    fn get(client_id: &str, client_secret: &str, code: &str, redirect_uri: &str) -> Self {
        let res = ureq::post(Self::URL)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", redirect_uri),
            ]);
            match res {
                Ok(token) => token.into_json::<Token>().unwrap(),
                Err(e) => {
                    panic!("Cant get Refresh Token: {e:?}")
                }
            }
            
    }

    fn refresh(client_id: &str, client_secret: &str, refresh_token: &str) -> Self {
        ureq::post(Self::URL)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ]).unwrap().into_json::<Token>().unwrap()
    }
}

/// This function will refresh our access_token with a the refresh token, and 
/// find a refresh token from a code if it cant find one
pub fn get_access_token() -> Result<String>{
    
    let twitch_client_id = std::env::var("TWITCH_CLIENT_ID")
        .expect("Please set the TWITCH_CLIENT_ID enviroment variable");

    let twitch_client_secret = std::env::var("TWITCH_CLIENT_SECRET")
        .expect("Please set the TWITCH_CLIENT_SECRET enviroment variable");

    let token = match std::env::var("TWITCH_REFRESH_TOKEN"){
        Ok(var) => {
            Token::refresh(
                &twitch_client_id, 
                &twitch_client_secret, 
                &var,
            )
        }
        Err(e) => {

            // Use the code we get manually to get a refresh token
            let twitch_code = match std::env::var("TWITCH_CODE"){
                Ok(code) => code,
                Err(e) => {
                    return Err(Error::EnvVarMissing(&"TWITCH_CODE also missing, please consult README.md to generate one"))
                }
            };

            // Use the refresh token to refresh
            let token = Token::get(
                &twitch_client_id, 
                &twitch_client_secret, 
                &twitch_code,
                &"http://localhost:8080"
            );

            // Fill in our refresh token to env if we didnt have one
            std::env::set_var(&"TWITCH_REFRESH_TOKEN", &token.refresh_token);

            // Refresh the access_token
            Token::refresh(
                &twitch_client_id, 
                &twitch_client_secret, 
                &token.refresh_token,
            )
        }
    };
    println!("{token:?}");
    Ok(token.access_token)
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get(){
    //     let token = Token::get();
    //     println!("{token:?}");
    // }
}