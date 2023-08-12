mod game;
mod ui;

use bevy::{
    input::common_conditions::input_toggle_active,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // input::common_conditions::input_toggle_active,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    text::TextSettings,
    window::WindowPlugin,
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Conway's Game Of Life".into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    // backends: Some(Backends::VULKAN),
                    ..default()
                },
            }),
        // LogDiagnosticsPlugin::default(),
        // FrameTimeDiagnosticsPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .insert_resource(TextSettings {
        allow_dynamic_font_size: true,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins((game::GamePlugin, ui::UIPlugin));
    app.run();
}
