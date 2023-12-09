/*
    Reference: https://github.com/bevyengine/bevy/blob/main/examples/ui/text.rs
*/

use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct GameStatsText;

const FPS_TEXT_SIZE: f32 = 20.;

pub fn hud_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle { 
                    font: font.clone(), 
                    font_size: FPS_TEXT_SIZE, 
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle { 
                    font: font, 
                    font_size: FPS_TEXT_SIZE, 
                    color: Color::ORANGE 
                }
            )
        ]),
        Hud,
        FpsText
    ));
}

pub fn hud_cleanup(
    mut commands: Commands,
    hud_q: Query<Entity, With<Hud>>
) {
    hud_q.for_each(|hud_entity| commands.entity(hud_entity).despawn());
}

pub fn text_update(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_q: Query<&mut Text, With<FpsText>>
) {
    let mut fps_text = fps_text_q.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(smoothed_fps) = fps.smoothed() {
            fps_text.sections[1].value = format!("{smoothed_fps:.2}");
        }
    }
}