use bevy::prelude::*;

use crate::asset_library::AssetLibrary;
use crate::logic::power::PowerCycleTracker;
use crate::GameState;

mod power;

use power::{power_view, PowerView, PowerViewState};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerViewState {
            tracker: PowerCycleTracker::new(2, 4, 0, 0),
        })
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(init))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(power_view));
    }
}

fn init(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    let style = TextStyle {
        font: asset_library.font("game"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Power Bowl\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Bowl 1: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Bowl 2: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Bowl 3: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Bowl G: 0\n".to_string(),
                        style: style,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PowerView);
}
