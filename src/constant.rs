pub fn get_avatar_url(user_id: &str) -> String{
    format!("https://www.roblox.com/headshot-thumbnail/image?userId={user_id}&width=48&height=48&format=png")
}
pub fn get_id_url(user_name: &str) -> String{
    format!("https://api.roblox.com/users/get-by-username?username={user_name}")
}