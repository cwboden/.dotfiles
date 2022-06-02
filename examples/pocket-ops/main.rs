use bevy::prelude::*;
use dotfiles::input::interaction::{Interactable, InteractionSource, InteractionState};
use dotfiles::input::select::{SelectPlugin, Selectable, SelectedEntity};

#[derive(Component)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct Highlighted {
    is_highlighted: bool,
}

impl Default for Highlighted {
    fn default() -> Self {
        Self {
            is_highlighted: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlayerColor {
    Yellow,
    Purple,
}

struct Turn(PlayerColor);

impl Turn {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            PlayerColor::Yellow => PlayerColor::Purple,
            PlayerColor::Purple => PlayerColor::Yellow,
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self(PlayerColor::Yellow)
    }
}

#[derive(Component)]
struct TurnText;

#[derive(Component, Default)]
struct Contents {
    piece: Option<PlayerColor>,
}

#[derive(Default)]
struct AssetLibrary {
    yellow: Handle<Image>,
    purple: Handle<Image>,
    pub font: Handle<Font>,
}

impl AssetLibrary {
    pub fn get_texture(&self, color: PlayerColor) -> Handle<Image> {
        match color {
            PlayerColor::Yellow => self.yellow.clone(),
            PlayerColor::Purple => self.purple.clone(),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pocket Ops".to_string(),
            width: 1080.0,
            height: 810.0,
            ..Default::default()
        })
        .init_resource::<Turn>()
        .init_resource::<AssetLibrary>()
        .add_plugins(DefaultPlugins)
        .add_plugin(SelectPlugin)
        .add_startup_system(create_camera)
        .add_startup_system(load_assets)
        .add_startup_system(create_board)
        .add_startup_system(create_ui)
        .add_system(highlight_selected_square)
        .add_system(place_piece_on_square)
        .run();
}

fn load_assets(mut asset_library: ResMut<AssetLibrary>, asset_server: Res<AssetServer>) {
    asset_library.yellow = asset_server.load("sprites/circle-yellow.png");
    asset_library.purple = asset_server.load("sprites/circle-purple.png");
}

fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource::default());
    commands.spawn_bundle(UiCameraBundle::default());
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
                        .insert(Selectable)
                        .insert(Highlighted::default())
                        .insert(Contents::default());
                }
            }
        });
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_library: ResMut<AssetLibrary>,
) {
    asset_library.font = asset_server.load("fonts/ClassicConsoleNeue.ttf");

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "To Move: Yellow",
                TextStyle {
                    font: asset_library.font.clone(),
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(TurnText);
}

fn highlight_selected_square(
    selected_square: Res<SelectedEntity>,
    mut query: Query<(Entity, &mut Sprite, &mut Highlighted), With<Selectable>>,
) {
    for (entity, mut sprite, mut highlighted) in query.iter_mut() {
        if Some(entity) == selected_square.entity {
            sprite.color = Color::TOMATO;
            highlighted.is_highlighted = true;
        } else {
            sprite.color = Color::GRAY;
            highlighted.is_highlighted = false;
        }
    }
}

fn place_piece_on_square(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut turn_state: ResMut<Turn>,
    mut query: Query<(Entity, &Highlighted, &mut Contents)>,
) {
    // Ignore non-mouse click events
    // XXX: Eventually we'll want to map this more dynamically to enable controller
    // remaps
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, highlighted, mut contents) in query.iter_mut() {
        // Check if we clicked on the highlighted square again
        if interaction_state
            .get_interaction_for_entity(entity)
            .is_some()
            && highlighted.is_highlighted
            && contents.piece.is_none()
        {
            contents.piece = Some(turn_state.0);
            println!("Added {:?} piece", turn_state.0);

            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_library.get_texture(turn_state.0),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(60.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..Default::default()
                    })
                    .insert(Name::new("Piece"));
            });

            turn_state.change();
        }
    }
}
