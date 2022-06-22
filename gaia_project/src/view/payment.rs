use bevy::prelude::*;

use crate::logic::payment::ResourcesState;
use crate::types::*;

#[derive(Component)]
pub struct GaugeView;

#[derive(Component)]
pub struct PowerView;

pub fn payment_view(
    resources_state: Res<ResourcesState>,
    mut power_view_query: Query<&mut Text, (With<PowerView>, Without<GaugeView>)>,
    mut gauge_view_query: Query<&mut Text, (With<GaugeView>, Without<PowerView>)>,
) {
    power_view_query.iter_mut().for_each(|mut text| {
        text.sections[1].value = format!("Bowl 1: {}\n", resources_state.power.get(PowerBowl::One));
        text.sections[2].value = format!("Bowl 2: {}\n", resources_state.power.get(PowerBowl::Two));
        text.sections[3].value =
            format!("Bowl 3: {}\n", resources_state.power.get(PowerBowl::Three));
        text.sections[4].value =
            format!("Bowl G: {}\n", resources_state.power.get(PowerBowl::Gaia));
    });

    gauge_view_query.iter_mut().for_each(|mut text| {
        text.sections[1].value = format!("Ore: {}\n", resources_state.ore.get());
        text.sections[2].value = format!("Knowledge: {}\n", resources_state.knowledge.get());
        text.sections[3].value = format!("Credits: {}\n", resources_state.credits.get());
        text.sections[4].value = format!("QIC: {}\n", resources_state.qic.get());
    });
}
