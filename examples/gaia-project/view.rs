use crate::asset_library::AssetLibrary;
use crate::power::PowerCycleTracker;
use crate::types::*;
use crate::GameState;
use bevy::prelude::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerViewState {
            tracker: PowerCycleTracker::new(2, 4, 0),
        })
        .add_system_set(SystemSet::on_enter(GameState::Running).with_system(init))
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(power_view)
                .with_system(input_monitor),
        )
        .add_event::<PowerEvent>();
    }
}

fn init(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    let style = TextStyle {
        font: asset_library.font("game"),
        font_size: 40.0,
        color: Color::WHITE,
    };
    let alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
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
                        style: style,
                    },
                ],
                ..Default::default()
            },
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

fn power_view(
    mut view_state: ResMut<PowerViewState>,
    mut events: EventReader<PowerEvent>,
    mut query: Query<&mut Text, With<PowerView>>,
) {
    for &event in events.iter() {
        match event {
            PowerEvent::Charge(amount) => view_state.tracker.charge(amount),
            PowerEvent::Reserve(amount) => view_state.tracker.reserve(amount).unwrap(),
            PowerEvent::Spend(amount) => view_state.tracker.spend(amount).unwrap(),
        }
    }

    for mut text in query.iter_mut() {
        text.sections[1].value = format!("Bowl 1: {}\n", view_state.tracker.get(PowerBowl::One));
        text.sections[2].value = format!("Bowl 2: {}\n", view_state.tracker.get(PowerBowl::Two));
        text.sections[3].value = format!("Bowl 3: {}\n", view_state.tracker.get(PowerBowl::Three));
        text.sections[4].value = format!("Bowl G: {}\n", view_state.tracker.get(PowerBowl::Gaia));
    }
}
