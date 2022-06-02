use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1080.,
            height: 810.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .run();
}
