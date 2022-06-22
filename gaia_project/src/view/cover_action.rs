use std::string::ToString;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::logic::cover_action::CoverActions;
use crate::types::*;

pub struct CoverActionViewState {
    pub actions: CoverActions,
}

#[derive(Component)]
pub struct CoverActionView;

pub fn cover_action_view(
    mut view_state: ResMut<CoverActionViewState>,
    mut events: EventReader<CoverActionEvent>,
    mut query: Query<&mut Text, With<CoverActionView>>,
) {
    for &event in events.iter() {
        match event {
            CoverActionEvent::Cover(action) => {
                view_state.actions.get_mut(action).cover().unwrap();
            }
            CoverActionEvent::Reset => {
                view_state.actions.reset();
            }
        }
    }

    for mut text in query.iter_mut() {
        let mut index = 0;
        CoverActionType::iter().for_each(|t| {
            index += 1;
            text.sections[index].value =
                format!("{}: {}\n", t.to_string(), view_state.actions.get(t).is_used);
        });
    }
}
