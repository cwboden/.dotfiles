use bevy::prelude::*;

use crate::logic::gauge::Gauges;
use crate::types::*;

pub struct GaugeViewState {
    pub gauges: Gauges,
}

#[derive(Component)]
pub struct GaugeView;

pub fn gauge_view(
    mut view_state: ResMut<GaugeViewState>,
    mut events: EventReader<GaugeEvent>,
    mut query: Query<&mut Text, With<GaugeView>>,
) {
    for &event in events.iter() {
        match event {
            GaugeEvent::Gain(cost) => {
                view_state.gauges.get_mut(cost.resource).add(cost.amount);
            }
            GaugeEvent::Spend(cost) => {
                view_state.gauges.get_mut(cost.resource).sub(cost.amount);
            }
        }
    }
}
