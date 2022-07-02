mod irc;
mod gw2;
mod twitch;


fn main() {

    let gw2_api_key = std::env::var("GW2_API_KEY")
        .expect("Please set the GW2_API_KEY enviroment variable");

    let twitch_username = std::env::var("TWITCH_USERNAME")
        .expect("Please set the TWITCH_USERNAME enviroment variable");

    let access_token = twitch::get_access_token()
        .expect("Could not refresh OATUH");

    // let gw2 = gw2::Api::new(&gw2_api_key)
    //     .expect("Could not connect to API");

    // println!("{gw2:#?}");

    // let link = rumble::Link::new()
    //     .expect("Could not set up memory mapped file");

    // gw2.where_am_i(link).expect("Error while getting position");

    join_irc(&twitch_username, &access_token)
}


fn join_irc(username: &str, api_key: &str){
    // Create a TCP connection
    let mut session = irc::Session::new("irc.chat.twitch.tv:6667");

    // Authenticate with the session
    session.authenticate(
        username, 
        api_key
    );
    // Join the channel called azphelumbrah
    session.join(username);

    // Send message to Channel with MSG
    session.send(username, "adam is super pretty you should subscribe now")
}

