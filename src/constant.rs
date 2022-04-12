pub const REQUEST_LIMIT: usize = 40;
pub const RETRY_LIMIT: usize = 100;
pub const PLACE_ID: &str = ""; //TODO: Add place id
pub const CAPTAIN_ID: &str = ""; //TODO: Add captain id
pub fn get_avatar_url(user_id: &str) -> String {
    format!("https://www.roblox.com/headshot-thumbnail/image?userId={user_id}&width=48&height=48&format=png")
}
pub fn get_id_url(user_name: &str) -> String {
    format!("https://api.roblox.com/users/get-by-username?username={user_name}")
}

pub fn get_game_instances(place_id: &str, i: usize) -> String {
    format!("www.roblox.com/games/getgameinstancesjson?placeId={place_id}&startIndex={i}")
}

pub fn get_join_url(place_id: &str, guid: &str) -> String {
    format!("https://www.roblox.com/home?placeid={place_id}&gameid={guid}")
}
