use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                title: "Dungeon Crawl".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
