use bevy::prelude::*;
use dotfiles::input::interaction::{Interactable, InteractionSource};
use dotfiles::input::select::{SelectPlugin, Selectable, SelectedEntity};

#[derive(Component)]
struct Coordinate {
    x: u8,
    y: u8,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pocket Ops".to_string(),
            width: 1080.0,
            height: 810.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SelectPlugin)
        .add_startup_system(create_camera)
        .add_startup_system(create_board)
        .add_system(highlight_selected_square)
        .run();
}

fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource::default());
}

fn create_board(mut commands: Commands) {
    let tile_size = 64.0;
    let padding = 2.0;
    let board_size = Vec2::new(tile_size * 3.0, tile_size * 3.0);
    let board_position = Vec3::new(-(board_size.x / 2.0), -(board_size.y / 2.0), 0.0);

    commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_translation(board_position))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(board_size.x / 2.0, board_size.y / 2.0, 0.0),
                    ..Default::default()
                })
                .insert(Name::new("Background"));

            for y in 0..=2 {
                for x in 0..=2 {
                    let x_offset = x as f32 * tile_size;
                    let y_offset = y as f32 * tile_size;

                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::GRAY,
                                custom_size: Some(Vec2::splat(tile_size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                x_offset + (tile_size / 2.0),
                                y_offset + (tile_size / 2.0),
                                1.0,
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new(format!("Tile ({x}, {y})")))
                        .insert(Coordinate {
                            x: x as u8,
                            y: y as u8,
                        })
                        .insert(Interactable {
                            bounding_box: (
                                Vec2::new(-(tile_size / 2.0), -(tile_size / 2.0)),
                                Vec2::new(tile_size / 2.0, tile_size / 2.0),
                            ),
                        })
                        .insert(Selectable);
                }
            }
        });
}

fn highlight_selected_square(
    selected_square: Res<SelectedEntity>,
    mut query: Query<(Entity, &mut Sprite), With<Selectable>>,
) {
    for (entity, mut sprite) in query.iter_mut() {
        if Some(entity) == selected_square.entity {
            sprite.color = Color::TOMATO;
        } else {
            sprite.color = Color::GRAY;
        }
    }
}
