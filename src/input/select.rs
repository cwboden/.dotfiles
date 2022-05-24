use bevy::prelude::*;

use crate::input::interaction::{InteractionPlugin, InteractionState};

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InteractionPlugin)
            .init_resource::<SelectedEntity>()
            .add_system(select_entity_system);
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Default)]
struct SelectedEntity {
    entity: Option<Entity>,
}

fn select_entity_system(
    mut selected_state: ResMut<SelectedEntity>,
    interaction_state: Res<InteractionState>,
    mouse_button_input: Res<Input<MouseButton>>,
    selectables: Query<Entity, With<Selectable>>,
) {
    // Ignore non-mouse click events
    // XXX: Eventually we'll want to map this more dynamically to enable controller
    // remaps
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for entity in selectables.iter() {
        if let Some(_) = interaction_state.get_interaction_for_entity(entity) {
            println!("Selected entity: {entity:?}");
            selected_state.entity = Some(entity);
        }
    }
}
