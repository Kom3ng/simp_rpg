use bevy::prelude::{App, AssetServer, Camera2d, Commands, Res, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::AppExtStates;
use simp_rpg::client::menu::MenuPlugin;
use simp_rpg::client::splash::SplashPlugin;
use simp_rpg::{FontHandles, GameState};

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let regular_font = asset_server.load("fonts/LXGWWenKai-Medium.ttf");
    let bold_font = asset_server.load("fonts/LXGWWenKai-Medium.ttf");

    // 将加载好的句柄作为资源插入到 App 中
    commands.insert_resource(FontHandles {
        regular: regular_font,
        bold: bold_font,
    });

    commands.spawn(Camera2d);
}