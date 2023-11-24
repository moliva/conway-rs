use bevy::{prelude::*, window::PrimaryWindow};

use conway_rs::{Grid, Simmetry, Stamp};

use crate::components::{
    GameTimer, Item, ItemBundle, SelectedOption, SimmetryResource, StampResource, COL_SIZE,
    ROW_SIZE,
};

const ALIVE_COLOR: Color = Color::ANTIQUE_WHITE;
const DEAD_COLOR: Color = Color::DARK_GRAY;

const CELL_SIZE: f32 = 10.;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn tickity(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
) {
    if timer.tick(time.delta()).finished() {
        grid.tick();
    }
}

pub fn spawn_grid(
    mut commands: Commands,
    grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
    selected_stamp_resource: Res<StampResource>,
    selected_simmetry_resource: Res<SimmetryResource>,
    mut query: Query<(Entity, &mut BackgroundColor), With<Item>>,
) {
    if query.is_empty() {
        commands
            .spawn(NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            aspect_ratio: Some(1.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::px(COL_SIZE, CELL_SIZE),
                            grid_template_rows: RepeatedGridTrack::px(ROW_SIZE as u16, CELL_SIZE),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        for row in grid.grid.iter() {
                            for &cell in row.iter() {
                                item_rect(builder, if cell { ALIVE_COLOR } else { DEAD_COLOR });
                            }
                        }
                    });

                builder
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        let button_style = Style {
                            width: Val::Px(150.0),
                            height: Val::Px(45.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        };

                        let button_text_style = TextStyle {
                            font_size: 25.0,
                            color: TEXT_COLOR,
                            ..default()
                        };

                        builder.spawn(TextBundle::from_section("Stamp", button_text_style.clone()));
                        for (text, stamp_resource) in [
                            ("Point", StampResource(Stamp::Point)),
                            ("Glider", StampResource(Stamp::Glider)),
                        ] {
                            let mut entity = builder.spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            });
                            entity.insert(stamp_resource);

                            if stamp_resource == *selected_stamp_resource {
                                entity.insert(SelectedOption);
                            }

                            entity.with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                        }

                        builder.spawn(TextBundle::from_section(
                            "Simmetry",
                            button_text_style.clone(),
                        ));
                        for (text, simmetry_resource) in [
                            ("None", SimmetryResource(Simmetry::None)),
                            ("X", SimmetryResource(Simmetry::X)),
                            ("Y", SimmetryResource(Simmetry::Y)),
                            ("XY", SimmetryResource(Simmetry::XY)),
                        ] {
                            let mut entity = builder.spawn(ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            });
                            entity.insert(simmetry_resource);

                            if simmetry_resource == *selected_simmetry_resource {
                                entity.insert(SelectedOption);
                            }

                            entity.with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                        }
                    });
            });
    } else {
        let mut iterator = query.iter_mut();

        for row in grid.grid.iter() {
            for &cell in row.iter() {
                let (_, mut background_color) = iterator.next().unwrap();
                background_color.0 = if cell { ALIVE_COLOR } else { DEAD_COLOR };
            }
        }
    }
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder.spawn(ItemBundle {
        node: NodeBundle {
            background_color: BackgroundColor(color),
            ..default()
        },
        ..default()
    });
}

pub fn handle_click(
    selected_stamp: Res<StampResource>,
    selected_simmetry: Res<SimmetryResource>,
    mut grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let x = to_grid(position.x);
        let y = to_grid(position.y);

        // TODO - avoid acting on an input that might be related to the menu - moliva - 2023/11/24
        if y > ROW_SIZE || x > COL_SIZE {
            return;
        }

        grid.stamp(selected_stamp.0, (y, x), selected_simmetry.0);
    } else {
        println!("Cursor is not in the game window.");
    }
}

pub fn pause_system(keys: Res<Input<KeyCode>>, mut timer: ResMut<GameTimer>) {
    if keys.just_released(KeyCode::Space) {
        if timer.paused() {
            timer.reset();
            timer.unpause();
        } else {
            timer.pause();
        }
    }
}

fn to_grid(n: f32) -> usize {
    (n / CELL_SIZE).floor() as usize
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), (With<SelectedOption>, With<T>)>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
