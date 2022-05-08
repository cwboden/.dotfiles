use bevy::prelude::*;

use crate::asset_library::AssetLibrary;
use crate::logic::cover_action::CoverActions;
use crate::logic::gauge::Gauges;
use crate::logic::power::PowerCycleTracker;
use crate::types::*;
use crate::GameState;

mod cover_action;
mod gauge;
mod power;

use cover_action::{cover_action_view, CoverActionView, CoverActionViewState};
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
        .insert_resource(CoverActionViewState {
            actions: CoverActions::new(),
        })
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(init))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(power_view))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(gauge_view))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(cover_action_view));
    }
}

fn init(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    let style = TextStyle {
        font: asset_library.font("game"),
        font_size: 24.0,
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
                        style: style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GaugeView);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Cover Actions:\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "GainThreePower: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "SingleTerraform: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "TwoKnowledge: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "SevenCredits: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "TwoOre: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "DoubleTerraform: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "ThreeKnowledge: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "PointsForPlanetTypes: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "RescoreFederationToken: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "GainTechTile: 0\n".to_string(),
                        style: style,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CoverActionView);
}
