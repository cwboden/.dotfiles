use crate::types::*;

pub trait TerraformingCost {
    fn terraforming_cost(&self) -> Amount;
}
