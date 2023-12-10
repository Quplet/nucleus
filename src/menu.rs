use bevy::prelude::*;

use crate::{GameState, GameStats};

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct LevelValue {
    level: i32
}

pub const NORMAL_BUTTON: Color = Color::BLACK;
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.1, 0.);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.15, 0.);

pub fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    let root_node = NodeBundle {
        style: Style {
            left: Val::Percent(5.),
            top: Val::Percent(5.),
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        border_color: Color::ORANGE.into(),
        ..default()
    };

    let title = TextBundle::from_section(
        "Nucleus", 
        TextStyle {
            font: font.clone(), 
            font_size: 72., 
            color: Color::ORANGE
        }
    ).with_style(
        Style {
            bottom: Val::Percent(40.),
            ..default()
        }
    ).with_text_alignment(TextAlignment::Center);

    let button_node = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.),
            top: Val::Percent(50.),
            width: Val::Percent(90.),
            height: Val::Percent(40.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        border_color: Color::ORANGE.into(),
        ..default()
    };

    commands.spawn((root_node, Menu))
    .with_children(|parent| {
        parent.spawn(title);
    }).with_children(|parent| {
        parent.spawn(button_node)
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::ORANGE.into(),
                    ..default()
                },
                LevelValue {
                    level: 0
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "0", 
                    TextStyle { font: font.clone(), font_size: 48., color: Color::ORANGE }
                ));
            });
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::ORANGE.into(),
                    ..default()
                },
                LevelValue {
                    level: 1
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "1", 
                    TextStyle { font: font.clone(), font_size: 48., color: Color::ORANGE }
                ));
            });
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::ORANGE.into(),
                    ..default()
                },
                LevelValue {
                    level: 2
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "2", 
                    TextStyle { font: font.clone(), font_size: 48., color: Color::ORANGE }
                ));
            });
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::ORANGE.into(),
                    ..default()
                },
                LevelValue {
                    level: 3
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "3", 
                    TextStyle { font: font.clone(), font_size: 48., color: Color::ORANGE }
                ));
            });
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        border: UiRect::all(Val::Px(3.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::ORANGE.into(),
                    ..default()
                },
                LevelValue {
                    level: 4
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "4", 
                    TextStyle { font: font.clone(), font_size: 48., color: Color::ORANGE }
                ));
            });
        });
    });
}

pub fn main_menu_cleanup(
    mut commands: Commands,
    menu_q: Query<Entity, With<Menu>>
) {
    menu_q.for_each(|menu_entity| commands.entity(menu_entity).despawn_recursive());
}

pub fn button_system(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor, &LevelValue), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_stats: ResMut<GameStats>
) {
    for (interaction, mut bg_color, level_value) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON.into();
                game_stats.level = level_value.level;
                next_state.set(GameState::SETUP);
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON.into();
            },
            Interaction::None => {
                *bg_color = NORMAL_BUTTON.into();
            }
        }
    }
}