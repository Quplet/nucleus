/*
    Reference: https://github.com/bevyengine/bevy/blob/main/examples/ui/text.rs
*/

use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};

use crate::{GameStats, level_manager::{LevelStats, calculate_grade}, menu::{PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON}, GameState};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct GameStatsText;

const HUD_TEXT_SIZE: f32 = 20.;

pub fn hud_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let static_text_style = TextStyle {
        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
        font_size: HUD_TEXT_SIZE,
        ..default()
    };
    let variable_text_style = TextStyle {
        font: static_text_style.font.clone(),
        font_size: HUD_TEXT_SIZE,
        color: Color::ORANGE,
        ..default()
    };

    // Fps ui
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS: ", static_text_style.clone()),
            TextSection::from_style(variable_text_style.clone())
        ]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                right: Val::Px(5.),
                ..default()
            }
        ),
        Hud,
        FpsText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Energy: ", static_text_style.clone()),
            TextSection::from_style(variable_text_style.clone()),
            TextSection::new("\nEnergy Grade: ", static_text_style.clone()),
            TextSection::from_style(variable_text_style.clone()),
            TextSection::new("\nRemaining Neutrons: ", static_text_style.clone()),
            TextSection::from_style(variable_text_style.clone()),
            TextSection::new("\nSimulation Speed: ", static_text_style.clone()),
            TextSection::from_style(variable_text_style.clone())
        ]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            }
        ),
        Hud,
        GameStatsText
    ));
}

pub fn setup_reset_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    let variable_text_style = TextStyle {
        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
        font_size: HUD_TEXT_SIZE,
        color: Color::ORANGE,
        ..default()
    };

    commands.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(5.),
                bottom: Val::Px(5.),
                width: Val::Px(75.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(2.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: Color::ORANGE.into(),
            ..default()
        },
        Hud
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section("Reset", variable_text_style));
    });
}

pub fn hud_cleanup(
    mut commands: Commands,
    hud_q: Query<Entity, With<Hud>>
) {
    hud_q.for_each(|hud_entity| commands.entity(hud_entity).despawn_recursive());
}

pub fn hud_text_update(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_q: Query<&mut Text, (With<FpsText>, Without<GameStatsText>)>,
    mut game_stats_text_q: Query<&mut Text, (With<GameStatsText>, Without<FpsText>)>,
    game_stats: Res<GameStats>,
    level_stats: Res<LevelStats>
) {
    let mut fps_text = fps_text_q.single_mut();
    let mut game_stats_text = game_stats_text_q.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(smoothed_fps) = fps.smoothed() {
            fps_text.sections[1].value = format!("{smoothed_fps:.0}");
        }
    }

    game_stats_text.sections[1].value = format!("{:.12} J", game_stats.score);
    game_stats_text.sections[3].value = format!("{}", calculate_grade(game_stats.score, level_stats.as_ref()));
    game_stats_text.sections[5].value = format!("{}", level_stats.num_neutrons);
    game_stats_text.sections[7].value = format!("{:.2}", game_stats.simulation_speed);
}

pub fn reset_button(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<Hud>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut bg_color) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON.into();
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