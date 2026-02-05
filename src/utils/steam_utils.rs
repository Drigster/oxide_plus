use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    #[serde(rename = "avatarFull")]
    pub avatar_full: String,
    #[serde(rename = "avatarMedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarIcon")]
    pub avatar_icon: String,
    #[serde(rename = "steamID")]
    pub username: String,
}

pub async fn get_profile_pic(steam_id: u64) -> Result<Profile, Box<dyn std::error::Error>> {
    let url = format!("https://steamcommunity.com/profiles/{}?xml=1", steam_id);
    let response = blocking::unblock(move || match ureq::get(url).call() {
        Ok(mut response) => Ok(response.body_mut().read_to_string()),
        Err(e) => Err(e),
    })
    .await??;

    let profile: Profile = quick_xml::de::from_str(response.as_str())?;
    Ok(profile)
}
