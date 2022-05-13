use bevy::prelude::*;

use crate::input::interaction::{Interactable, InteractionState};

/// Allows [`Interactable`] objects tagged with [`Draggable`] to be dragged and
/// dropped around the screen using the cursor.
pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionState>()
            .init_resource::<HoveredSnapPoint>()
            .add_system(drag_system)
            .add_system(drag_object_system)
            .add_system(select_snap_point_system)
            .add_system(drop_object_system);
    }
}

/// The way the entity should respond when dropped after being dragged.
pub enum DropStrategy {
    /// Leave the dropped entity at it's current position, wherever that may be.
    Leave,

    /// Align the dropped entity to a [`SnapPoint`] or reset it to it's original
    /// position.
    SnapOrReset,
}

/// Tag-only struct indicating that an object can be dragged and dropped using
/// the cursor. The object must also be marked as [`Interactable`] in order for
/// the cursor to move it around.
#[derive(Component)]
pub struct Draggable {
    pub drop_strategy: DropStrategy,
}

/// Tag added to entities that are currently being dragged around the screen.
#[derive(Component, Default)]
struct Dragged {
    pub translation: Vec2,
    pub origin: Vec2,
}

/// Moves [`Dragged`] objects around the screen based on the location of the
/// cursor.
fn drag_system(
    interaction_state: Res<InteractionState>,
    mut dragged_objects: Query<(&mut Transform, &mut Dragged, &GlobalTransform)>,
) {
    for (mut transform, dragged, global_transform) in dragged_objects.iter_mut() {
        let parent_matrix = global_transform
            .compute_matrix()
            .mul_mat4(&transform.compute_matrix().inverse());
        let global_hook_translation = (interaction_state.cursor_position + dragged.translation)
            .extend(transform.translation.z);

        transform.translation = parent_matrix
            .inverse()
            .transform_point3(global_hook_translation);
    }
}

/// Marks [`Draggable`] entities as [`Dragged`] when their bounding box is
/// clicked by the mouse.
fn drag_object_system(
    interaction_state: Res<InteractionState>,
    mouse_button_input: Res<Input<MouseButton>>,
    draggables: Query<(Entity, &Draggable, &GlobalTransform), With<Interactable>>,
    mut commands: Commands,
) {
    // Ignore non-mouse click events
    // XXX: Eventually we'll want to map this more dynamically to enable controller
    // remaps
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, _draggable, global_transform) in draggables.iter() {
        if let Some(interaction) = interaction_state.get_interaction_for_entity(entity) {
            let translation = global_transform.translation.truncate() - interaction.cursor_position;

            commands.entity(entity).insert(Dragged {
                translation,
                origin: global_transform.translation.truncate(),
            });
        }
    }
}

#[derive(Component, Default)]
struct HoveredSnapPoint {
    point: Option<SnapPoint>,
}

fn select_snap_point_system(
    interaction_state: Res<InteractionState>,
    mut hovered_snap_point: ResMut<HoveredSnapPoint>,
    snap_points: Query<(Entity, &SnapPoint), With<Interactable>>,
) {
    hovered_snap_point.point = None;
    println!("Cleared selected SnapPoint");

    for (entity, snap_point) in snap_points.iter() {
        if let Some(_) = interaction_state.get_interaction_for_entity(entity) {
            hovered_snap_point.point = Some(snap_point.clone());
            println!("Selected SnapPoint");
            break;
        }
    }
}

/// A point to snap [`Draggable`] entities with the `SnapOrReset`
/// [`DropStrategy`]
#[derive(Clone, Component, Default)]
pub struct SnapPoint {
    pub point: Vec2,
}

/// Removes the [`Dragged`] tag when the mouse button is released.
fn drop_object_system(
    mouse_button_input: Res<Input<MouseButton>>,
    hovered_snap_point: Res<HoveredSnapPoint>,
    mut draggables: Query<(Entity, &Draggable, &Dragged, &mut Transform), With<Interactable>>,
    mut commands: Commands,
) {
    // Ignore non-mouse click events
    // XXX: Eventually we'll want to map this more dynamically to enable controller
    // remaps
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (entity, draggable, dragged, mut transform) in draggables.iter_mut() {
        match draggable.drop_strategy {
            DropStrategy::Leave => {
                // Do nothing
            }
            DropStrategy::SnapOrReset => match &hovered_snap_point.point {
                None => {
                    println!("No hovered SnapPoint, returning to origin");
                    transform.translation = dragged.origin.extend(transform.translation.z);
                }
                Some(point) => {
                    println!("Aligning to hovered SnapPoint");
                    transform.translation = point.point.extend(transform.translation.z);
                }
            },
        }

        commands.entity(entity).remove::<Dragged>();
    }
}
