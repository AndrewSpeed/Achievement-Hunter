use std::collections::HashMap;

use anyhow::{bail, ensure, Context, Result};
use dialoguer::FuzzySelect;

use crate::settings::Settings;
use crate::steam::{Achievement, Client, GameAchievement, PlayerGameAchievement, UserGame};

fn merge_user_and_game_achievements(
    user_achievements: &Vec<PlayerGameAchievement>,
    game_achievements: &Vec<GameAchievement>,
) -> Vec<Achievement> {
    let mut api_name_to_achievement_mapping: HashMap<&str, Achievement> = HashMap::new();

    for it in user_achievements.iter().zip(game_achievements.iter()) {
        api_name_to_achievement_mapping
            .entry(&it.0.achievement_api_name)
            .and_modify(|ach| ach.merge_player_achievement(&it.0))
            .or_insert(Achievement::from(it.0));

        api_name_to_achievement_mapping
            .entry(&it.1.api_name)
            .and_modify(|ach| ach.merge_game_achievement(&it.1))
            .or_insert(Achievement::from(it.0));
    }

    api_name_to_achievement_mapping
        .into_values()
        .collect::<Vec<Achievement>>()
}

fn get_game_achievements(
    client: &Client,
    user_id: &str,
    game: &UserGame,
) -> Result<Vec<Achievement>> {
    let player_achievements = client
        .get_user_game_achievements(user_id, game.id)
        .context("Failed to retrieve user's achievement for game")?;

    let game_achievements = client
        .get_game_achievements(game.id)
        .context("Failed to get list of game achievements")?;

    ensure!(
        player_achievements.len() == game_achievements.len(),
        format!(
            "Player achievements size {}, game achievements size {}",
            player_achievements.len(),
            game_achievements.len()
        )
    );

    Ok(merge_user_and_game_achievements(
        &player_achievements,
        &game_achievements,
    ))
}

pub fn get_achievements(settings: &Settings) -> Result<Vec<Achievement>> {
    let user_id = &settings.steam.user_id;

    let client = Client {
        api_key: &settings.steam.api_key,
    };
    let games = client.get_user_games(&user_id)?;
    let game_names = games
        .iter()
        .map(|game| game.name.as_str())
        .collect::<Vec<&str>>();

    let selection = FuzzySelect::new()
        .with_prompt("What game do you want to view achievements for?")
        .items(&game_names)
        .interact_opt()?;

    match selection {
        Some(index) => {
            let selected_game = &games[index];
            let achievements = get_game_achievements(&client, user_id, selected_game)?;
            Ok(achievements)
        }
        None => {
            bail!("You did not choose a game.")
        }
    }
}
