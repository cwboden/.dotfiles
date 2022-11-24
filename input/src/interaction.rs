use bevy::ecs::event::{Events, ManualEventReader};
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionState>()
            .add_system_to_stage(CoreStage::PostUpdate, interaction_state_system)
            .add_system_to_stage(CoreStage::PostUpdate, interaction_system);
    }
}

/// The "global" state of the interaction system. Used to track the current
/// mouse position and all interactions created by the system.
#[derive(Default)]
pub struct InteractionState {
    pub cursor_position: Vec2,
    interactions: Vec<Interaction>,
}

impl InteractionState {
    pub fn get_interaction_for_entity(&self, entity: Entity) -> Option<&Interaction> {
        self.interactions.iter().find(|i| i.entity == entity)
    }
}

/// An individual interaction triggered when the position of the cursor overlaps
/// the bounding box of an [`Interactable`] `Entity`.
pub struct Interaction {
    pub entity: Entity,
    pub cursor_position: Vec2,
}

/// Attached to cameras that should be interacted from.
#[derive(Component, Default)]
pub struct InteractionSource {
    cursor_events: ManualEventReader<CursorMoved>,
}

/// Determines whether any objects have been interacted with whenever the cursor
/// is moved.
fn interaction_state_system(
    mut interaction_state: ResMut<InteractionState>,
    cursor_moved: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    mut sources: Query<(&mut InteractionSource, &GlobalTransform, Option<&Camera>)>,
) {
    for (mut interaction_source, transform, camera) in sources.iter_mut() {
        let (window_id, cursor_position) =
            match interaction_source.cursor_events.iter(&cursor_moved).last() {
                Some(event) => (event.id, event.position),
                None => {
                    // Do nothing
                    return;
                }
            };

        let projection_matrix = match camera {
            Some(camera) => camera.projection_matrix,
            None => unimplemented!(),
        };

        match windows.get(window_id) {
            Some(window) => {
                let screen_size = Vec2::from([window.width(), window.height()]);
                let relative_cursor_position =
                    (cursor_position / screen_size) * 2.0 - Vec2::new(1.0, 1.0);
                let camera_matrix = transform.compute_matrix();

                let relative_matrix: Mat4 = camera_matrix * projection_matrix.inverse();
                let cursor_position = relative_matrix
                    .transform_point3(relative_cursor_position.extend(1.0))
                    .truncate();

                interaction_state.cursor_position = cursor_position;
            }
            None => unimplemented!(),
        }
    }
}

/// An object that can be interacted with using the [`InteractionPlugin`]
#[derive(Component)]
pub struct Interactable {
    pub bounding_box: (Vec2, Vec2),
}

pub fn rectangle_contains_point(rectangle: (Vec2, Vec2), point: Vec2) -> bool {
    (rectangle.0.x..=rectangle.1.x).contains(&point.x)
        && (rectangle.0.y..=rectangle.1.y).contains(&point.y)
}

/// Creates an interaction whenever an [`Interactable`] entity has been moused
/// over by the cursor.
fn interaction_system(
    mut interaction_state: ResMut<InteractionState>,
    interactables: Query<(Entity, &GlobalTransform, &Interactable)>,
) {
    interaction_state.interactions.clear();

    for (entity, transform, interactable) in interactables.iter() {
        let relative_cursor_position = (interaction_state.cursor_position
            - transform.translation.truncate())
            / transform.scale.truncate();

        if rectangle_contains_point(interactable.bounding_box, relative_cursor_position) {
            let new_interaction = Interaction {
                entity,
                cursor_position: interaction_state.cursor_position,
            };
            interaction_state.interactions.push(new_interaction);
        }
    }
}
