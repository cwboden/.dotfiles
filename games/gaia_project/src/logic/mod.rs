use bevy::prelude::*;

use crate::GameState;

pub mod cover_action;
pub mod federation;
pub mod gauge;
pub mod input;
pub mod payment;
pub mod power;
pub mod research;
pub mod sector;
pub mod terraforming;

use federation::{federation_token_system, FederationTokens};
use payment::{payment_system, ResourcesState};

use self::input::InputPlugin;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourcesState::new())
            .insert_resource(FederationTokens::new())
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(payment_system))
            .add_system_set(
                SystemSet::on_update(GameState::Running).with_system(federation_token_system),
            )
            .add_plugin(InputPlugin);
    }
}
