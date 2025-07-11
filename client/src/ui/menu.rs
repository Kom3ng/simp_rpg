use bevy::app::App;
use bevy::prelude::*;
use crate::global::{FontHandles, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), menu_teardown);
    }
}

#[derive(Component)]
struct MenuUIRoot;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

fn menu_teardown(mut commands: Commands, menu_ui: Query<Entity, With<MenuUIRoot>>) {
    for entity in menu_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn menu_setup(mut commands: Commands, font_handles: Res<FontHandles>) {
    debug!("setup menu");
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        MenuUIRoot,
        children![
            (
                Text::new("Simple RPG"),
                TextFont {
                    font_size: 80.0,
                    font: font_handles.regular.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            common_button("开始", MenuButtonAction::Play, &font_handles),
            common_button("设置", MenuButtonAction::Settings, &font_handles),
            common_button("退出", MenuButtonAction::Quit, &font_handles)
        ],
    ));
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match menu_button_action {
                    MenuButtonAction::Play => {
                        // 切换到游戏状态
                        next_game_state.set(GameState::Game);
                    }
                    MenuButtonAction::Settings => {}
                    MenuButtonAction::Quit => {
                        // 发送退出应用事件
                        app_exit_writer.write(AppExit::Success);
                    }
                }
            }
            _ => {}
        }
    }
}

fn common_button(text: &str,action: MenuButtonAction ,font_handles: &Res<FontHandles>) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(Color::WHITE),
        action,
        children![(
                    Text::new(text),
                    TextColor(Color::BLACK),
                    TextFont {
                        font: font_handles.regular.clone(),
                        ..default()
                    },
                )],
    )
}
