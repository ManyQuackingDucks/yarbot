pub const REQUEST_LIMIT: usize = 20;
pub const RETRY_LIMIT: usize = 100;
pub const PLACE_ID: &str = "3411100258"; //TODO: Add place id
pub fn get_avatar_url(user_id: &str) -> String {
    format!("https://www.roblox.com/headshot-thumbnail/image?userId={user_id}&width=48&height=48&format=png")
}
pub fn get_id_url(user_name: &str) -> String {
    format!("https://api.roblox.com/users/get-by-username?username={user_name}")
}

pub fn get_game_instances(place_id: &str, i: usize) -> String {
    format!("https://web.roblox.com/games/getgameinstancesjson?placeId={place_id}&startIndex={i}")
}

pub fn get_join_url(place_id: &str, guid: &str) -> String {
    format!("https://web.roblox.com/home?placeID={place_id}&gameID={guid}")
}

pub fn get_game_url() -> &'static str {
    "https://presence.roblox.com/v1/presence/users"
}
