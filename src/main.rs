use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 3 🦀".into(),
                        mode: if cfg!(debug_assertions) {
                            bevy::window::WindowMode::Windowed
                        } else {
                            bevy::window::WindowMode::BorderlessFullscreen
                        },
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(game::Plugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}
