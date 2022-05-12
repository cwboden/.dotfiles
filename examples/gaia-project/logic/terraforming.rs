use bevy::prelude::*;

use crate::types::*;

pub trait TerraformingCost {
    fn terraforming_cost(&self) -> Amount;
}

#[cfg(test)]
mod tests {}
