use bevy::prelude::*;
use input::drag::{DragPlugin, Draggable, DropStrategy, SnapPoint};
use input::interaction::{Interactable, InteractionPlugin, InteractionSource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractionPlugin)
        .add_plugin(DragPlugin)
        .add_startup_system(setup.system())
        .run();
}

#[derive(Component)]
struct Relic;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource::default());

    // Create a Relic card for testing drag-and-drop
    let relic_back_texture = asset_server.load("odk/relic card back.png");
    let relic_back_atlas = texture_atlases.add(TextureAtlas::from_grid(
        relic_back_texture,
        Vec2::new(719., 719.),
        1, // columns
        1, // rows
    ));

    // Create a snappable card and a standard drag-and-drop one
    for drop_strategy in [DropStrategy::SnapOrReset, DropStrategy::Leave] {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: relic_back_atlas.clone(),
                transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)),
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            })
            .insert(Interactable {
                bounding_box: (Vec2::new(-215.7, -215.7), Vec2::new(215.7, 215.7)),
            })
            .insert(Draggable { drop_strategy })
            .insert(Relic);
    }

    // Create the snap point for the snappable card
    let snap_point_texture = asset_server.load("snap-point.png");
    let snap_point_atlas = texture_atlases.add(TextureAtlas::from_grid(
        snap_point_texture,
        Vec2::new(32., 32.),
        1, // columns
        1, // rows
    ));

    for i in 0..3 {
        let x = 200. - (200. * i as f32);
        let transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(7., 7., 7.),   // scale
            Quat::default(),         // rotation
            Vec3::new(x, -200., 3.), // translation
        ));

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: snap_point_atlas.clone(),
                transform,
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            })
            .insert(SnapPoint {
                point: Vec2::new(x, -200.),
            })
            .insert(Interactable {
                bounding_box: (Vec2::new(-16., -16.), Vec2::new(16., 16.)),
            });
    }
}
