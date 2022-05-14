use bevy::prelude::*;

use crate::asset_library::AssetLibrary;
use crate::logic::cover_action::CoverActions;
use crate::logic::research::ResearchTracks;
use crate::GameState;

pub mod cover_action;
pub mod federation;
pub mod payment;
pub mod research;

use cover_action::{cover_action_view, CoverActionView, CoverActionViewState};
use federation::{federation_token_view, FederationTokenView};
use payment::{payment_view, GaugeView, PowerView};
use research::{research_view, ResearchView, ResearchViewState};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoverActionViewState {
            actions: CoverActions::new(),
        })
        .insert_resource(ResearchViewState {
            tracks: ResearchTracks::new(),
        })
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(init))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(cover_action_view))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(research_view))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(payment_view))
        .add_system_set(
            SystemSet::on_update(GameState::Running).with_system(federation_token_view),
        );
    }
}

fn spawn_power_bowl_assets(commands: &mut Commands, style: TextStyle) {
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

fn spawn_gauges_assets(commands: &mut Commands, style: TextStyle) {
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

fn spawn_cover_actions_assets(commands: &mut Commands, style: TextStyle) {
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
                        style: style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CoverActionView);
}

fn spawn_research_assets(commands: &mut Commands, style: TextStyle) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Research\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Terraforming: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Flight: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "ArtificialIntelligence: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Gaiaforming: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Economics: 0\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "Science: 0\n".to_string(),
                        style: style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ResearchView);
}

fn spawn_federation_tokens_assets(commands: &mut Commands, style: TextStyle) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Federation Tokens\n".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FederationTokenView);
}

fn init(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    let style = TextStyle {
        font: asset_library.font("game"),
        font_size: 24.0,
        color: Color::WHITE,
    };

    spawn_power_bowl_assets(&mut commands, style.clone());
    spawn_gauges_assets(&mut commands, style.clone());
    spawn_cover_actions_assets(&mut commands, style.clone());
    spawn_research_assets(&mut commands, style.clone());
    spawn_federation_tokens_assets(&mut commands, style.clone());
}
