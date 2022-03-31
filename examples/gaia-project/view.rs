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
        .add_system_to_stage(CoreStage::PostUpdate, power_view)
        .add_system(input_monitor)
        .add_event::<PowerEvent>();
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

#[derive(Copy, Clone, Eq, PartialEq)]
enum PowerEvent {
    Charge(u8),
    Reserve(u8),
    Spend(u8),
}

// TODO: This should eventually become it's own plugin
fn input_monitor(input: Res<Input<KeyCode>>, mut events: EventWriter<PowerEvent>) {
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
            events.send(PowerEvent::Charge(amount));
        }
    }

    if input.just_pressed(KeyCode::R) {
        events.send(PowerEvent::Reserve(1));
    }

    if input.just_pressed(KeyCode::S) {
        events.send(PowerEvent::Spend(4));
    }
}

pub struct PowerViewState {
    pub tracker: PowerCycleTracker,
}

#[derive(Component)]
pub struct PowerView;

fn power_view(mut view_state: ResMut<PowerViewState>, mut events: EventReader<PowerEvent>) {
    for &event in events.iter() {
        match event {
            PowerEvent::Charge(amount) => view_state.tracker.charge(amount),
            PowerEvent::Reserve(amount) => view_state.tracker.reserve(amount).unwrap(),
            PowerEvent::Spend(amount) => view_state.tracker.spend(amount).unwrap(),
        }
    }

    println!("Power Bowl 1: {}", view_state.tracker.get(PowerBowl::One));
    println!("Power Bowl 2: {}", view_state.tracker.get(PowerBowl::Two));
    println!("Power Bowl 3: {}", view_state.tracker.get(PowerBowl::Three));
    println!("Power Bowl G: {}", view_state.tracker.get(PowerBowl::Gaia));
}
