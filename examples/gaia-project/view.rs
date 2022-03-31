use crate::asset_library::AssetLibrary;
use crate::power::PowerCycleTracker;
use crate::types::*;
use bevy::prelude::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerViewState {
            tracker: PowerCycleTracker::new(2, 4, 0),
        })
        .add_startup_system(init)
        .add_system_to_stage(CoreStage::PostUpdate, power_view);
    }
}

fn init(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_library.font("game"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, 55.0, 0.5),
            ..Default::default()
        })
        .insert(PowerView);
}

pub struct PowerViewState {
    pub tracker: PowerCycleTracker,
}

#[derive(Component)]
pub struct PowerView;

fn power_view(mut view_state: ResMut<PowerViewState>, input: Res<Input<KeyCode>>) {
    for &(key, amount) in [
        (KeyCode::Key1, 1),
        (KeyCode::Key2, 2),
        (KeyCode::Key3, 3),
        (KeyCode::Key4, 4),
        (KeyCode::Key5, 5),
        (KeyCode::Key6, 6),
        (KeyCode::Key7, 7),
        (KeyCode::Key8, 8),
        (KeyCode::Key9, 9),
    ]
    .iter()
    {
        if input.just_pressed(key) {
            view_state.tracker.charge(amount);
        }
    }

    // Would be nice to extract this into an event-driven input sytem
    if input.just_pressed(KeyCode::R) {
        view_state.tracker.reserve(1).unwrap();
    }

    if input.just_pressed(KeyCode::S) {
        view_state.tracker.spend(4).unwrap();
    }

    println!("Power Bowl 1: {}", view_state.tracker.get(PowerBowl::One));
    println!("Power Bowl 2: {}", view_state.tracker.get(PowerBowl::Two));
    println!("Power Bowl 3: {}", view_state.tracker.get(PowerBowl::Three));
    println!("Power Bowl G: {}", view_state.tracker.get(PowerBowl::Gaia));
}
