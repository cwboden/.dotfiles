use bevy::prelude::*;

use crate::GameState;

pub mod cover_action;
pub mod gauge;
pub mod input;
pub mod payment;
pub mod power;
pub mod research;

use input::InputPlugin;
use payment::{payment_system, ResourcesState};

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourcesState::new())
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(payment_system))
            .add_plugin(InputPlugin);
    }
}
