use bevy::log::debug;
use bevy::prelude::{in_state, App, IntoScheduleConfigs, NextState, Plugin, ResMut, Update};
use crate::GameState;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fade.run_if(in_state(GameState::Splash)));       
    }
}
fn fade(
    mut game_state: ResMut<NextState<GameState>>,
){
    debug!("quiting splash");
    game_state.set(GameState::Menu);
}