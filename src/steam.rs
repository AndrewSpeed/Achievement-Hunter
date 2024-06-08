use ::chrono::{DateTime, Utc};
use anyhow::{Context, Error};
use chrono::serde::{ts_seconds, ts_seconds_option};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Client<'a> {
    pub api_key: &'a str,
}

fn deserialize_u8_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: u8 = serde::de::Deserialize::deserialize(deserializer)?;

    match value {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(
            &value.to_string(),
            &["1", "0"],
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct Achievement {
    pub api_name: String,
    pub display_name: String,
    pub hidden: bool,
    pub description: Option<String>,
    pub achieved: bool,
    pub achieved_at: Option<DateTime<Utc>>,
}

impl Achievement {
    pub fn merge_game_achievement(&mut self, achievement: &GameAchievement) -> () {
        *self = Self {
            api_name: achievement.api_name.clone(),
            description: achievement.description.clone(),
            display_name: achievement.display_name.clone(),
            hidden: achievement.hidden,
            ..*self
        };
    }

    pub fn merge_player_achievement(&mut self, achievement: &PlayerGameAchievement) -> () {
        *self = Self {
            api_name: achievement.achievement_api_name.clone(),
            achieved: achievement.achieved,
            achieved_at: achievement.achieved_at,
            description: self.description.clone(),
            display_name: self.display_name.clone(),
            hidden: self.hidden,
        }
    }
}

impl From<&GameAchievement> for Achievement {
    fn from(achievement: &GameAchievement) -> Self {
        Achievement {
            api_name: achievement.api_name.clone(),
            display_name: achievement.display_name.clone(),
            description: achievement.description.clone(),
            hidden: achievement.hidden,
            achieved: false,
            achieved_at: None,
        }
    }
}

impl From<&PlayerGameAchievement> for Achievement {
    fn from(achievement: &PlayerGameAchievement) -> Self {
        Achievement {
            api_name: achievement.achievement_api_name.clone(),
            display_name: String::default(),
            description: None,
            hidden: false,
            achieved: achievement.achieved,
            achieved_at: achievement.achieved_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GameAchievement {
    ///
    ///{
    ///  "defaultvalue": 0,
    ///  "description": "Unlock all Skills",
    ///  "displayName": "Superior Spider-Man",
    ///  "hidden": 0,
    ///  "icon": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/apps/1817070/5b4727325d1f7d46c6fbd2c5bbd8f4f59ec6c361.jpg",
    ///  "icongray": "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/apps/1817070/641a64fe768fa6182317e0e8ce01a21f260d7efb.jpg",
    ///  "name": "superiorspiderman"
    ///},
    ///
    // description and hidden are mutually exclusive?
    pub description: Option<String>,
    #[serde(alias = "displayName")]
    pub display_name: String,
    #[serde(deserialize_with = "deserialize_u8_to_bool")]
    pub hidden: bool,
    #[serde(alias = "name")]
    pub api_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GameStats {
    achievements: Vec<GameAchievement>,
}

#[derive(Debug, Deserialize)]
pub struct GameData {
    #[serde(alias = "availableGameStats")]
    stats: GameStats,
}

#[derive(Debug, Deserialize)]
pub struct GameAchievementsResponse {
    ///
    ///{
    ///  "game": {
    ///    "availableGameStats": {
    ///      "achievements": [GameAchievement]
    ///    }
    ///  }
    ///}
    ///
    game: GameData,
}

#[derive(Debug, Deserialize)]
pub struct UserGame {
    ///{
    ///  "appid": 1510,
    ///  "name": "Uplink",
    ///  "playtime_forever": 405,
    ///  "img_icon_url": "3f24e709b546f06641586190a6a84be791161830",
    ///  "playtime_windows_forever": 0,
    ///  "playtime_mac_forever": 0,
    ///  "playtime_linux_forever": 0,
    ///  "playtime_deck_forever": 0,
    ///  "rtime_last_played": 1400429710,
    ///  "playtime_disconnected": 0
    ///},
    ///
    #[serde(alias = "appid")]
    pub id: u32,
    pub name: String,
    #[serde(alias = "rtime_last_played", with = "ts_seconds")]
    pub last_playtime: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UserGames {
    games: Vec<UserGame>,
}

#[derive(Debug, Deserialize)]
pub struct UserGamesResponse {
    ///
    ///{
    ///  "response: {
    ///    "game_count": 23,
    ///    "games: UserGames
    ///  }
    ///}
    ///
    response: UserGames,
}

#[derive(Debug, Deserialize)]
pub struct PlayerGameAchievement {
    #[serde(alias = "apiname")]
    pub achievement_api_name: String,
    #[serde(deserialize_with = "deserialize_u8_to_bool")]
    pub achieved: bool,
    #[serde(alias = "unlocktime", with = "ts_seconds_option")]
    pub achieved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStats {
    #[serde(alias = "steamID")]
    pub user_id: String,
    #[serde(alias = "gameName")]
    pub game_name: String,
    pub achievements: Vec<PlayerGameAchievement>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerGameAchievementsResponse {
    ///
    ///{
    ///  "playerstats: {
    ///    "steamID": "",
    ///    "gameName": "",
    ///    "achievements": [GameAchievement...]
    ///  }
    ///}
    #[serde(alias = "playerstats")]
    pub player_stats: PlayerStats,
}

impl Client<'_> {
    fn create_request(&self, url: &str) -> reqwest::blocking::RequestBuilder {
        reqwest::blocking::Client::new()
            .get(url)
            .query(&[("key", &self.api_key)])
    }

    pub fn get_game_achievements(&self, game_id: u32) -> Result<Vec<GameAchievement>, Error> {
        let url = "https://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/";

        let response = self
            .create_request(url)
            .query(&[("appid", game_id)])
            .send()
            .context("Steam API request failed")?
            .json::<GameAchievementsResponse>()
            .context("Converting game achievements response failed")?;

        Ok(response.game.stats.achievements)
    }

    pub fn get_user_games(&self, user_id: &str) -> Result<Vec<UserGame>, Error> {
        let url = "http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001";

        let response = self
            .create_request(url)
            .query(&[("steamid", user_id), ("include_appinfo", "true")])
            .send()
            .context("Steam API request failed")?
            .json::<UserGamesResponse>()
            .context("Converting user games response failed")?;

        Ok(response.response.games)
    }

    pub fn get_user_game_achievements(
        &self,
        user_id: &str,
        game_id: u32,
    ) -> Result<Vec<PlayerGameAchievement>, Error> {
        let url = "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001";

        let response = self
            .create_request(url)
            .query(&[("steamid", user_id), ("appid", &game_id.to_string())])
            .send()
            .context("Steam player game achievements request failed")?
            .json::<PlayerGameAchievementsResponse>()
            .context("Converting player game achievements response failed")?;

        Ok(response.player_stats.achievements)
    }
}
