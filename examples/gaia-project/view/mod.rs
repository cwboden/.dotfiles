use bevy::prelude::*;

use crate::asset_library::AssetLibrary;
use crate::logic::gauge::Gauges;
use crate::logic::power::PowerCycleTracker;
use crate::types::*;
use crate::GameState;

mod gauge;
mod power;

use gauge::{gauge_view, GaugeView, GaugeViewState};
use power::{power_view, PowerView, PowerViewState};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerViewState {
            tracker: PowerCycleTracker::new(2, 4, 0, 0),
        })
        .insert_resource(GaugeViewState {
            gauges: Gauges::new(),
        })
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(init))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(power_view))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(gauge_view));
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
                        style: style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PowerView);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Gauges\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Ore: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Knowledge: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Credits: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "QIC: 0\n".to_string(),
                        style: style,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GaugeView);
}
