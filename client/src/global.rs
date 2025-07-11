use bevy::prelude::{Font, Handle, Resource, States};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
pub struct FontHandles {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
}
