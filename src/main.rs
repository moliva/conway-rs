use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};

use conway_rs::{Grid, Stamp};

const COL_SIZE: usize = 127;
const ROW_SIZE: usize = 71;

const TICK_SECONDS: f32 = 0.25;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/**
 * The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, live or dead (or populated and unpopulated, respectively). Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:
 *
 * 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
 * 2. Any live cell with two or three live neighbours lives on to the next generation.
 * 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
 * 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
 *
 * The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules simultaneously to every cell in the seed, live or dead; births and deaths occur simultaneously, and the discrete moment at which this happens is sometimes called a tick.[nb 1] Each generation is a pure function of the preceding one. The rules continue to be applied repeatedly to create further generations.
 */
fn main() {
    let mut grid = Grid::<COL_SIZE, ROW_SIZE>::sized();

    // set live cells
    grid.stamp(Stamp::Point, (9, 0));
    grid.stamp(Stamp::Block, (4, 4));
    grid.stamp(Stamp::Blinker, (8, 7));
    grid.stamp(Stamp::Glider, (0, 7));
    grid.stamp(Stamp::BeeHive, (4, 15));
    grid.stamp(Stamp::Tub, (10, 15));

    App::new()
        .insert_resource(grid)
        .insert_resource(GameTimer(Timer::from_seconds(
            TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LogDiagnosticsPlugin::default())
        // .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (spawn_grid, tickity))
        // Adds the plugins for each state
        // .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}

const ALIVE_COLOR: Color = Color::ANTIQUE_WHITE;
const DEAD_COLOR: Color = Color::DARK_GRAY;

const CELL_SIZE: f32 = 10.;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn tickity(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
) {
    if timer.tick(time.delta()).finished() {
        grid.tick();
    }
}

fn spawn_grid(
    mut commands: Commands,
    grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
    query: Query<Entity, With<Node>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::min_content()],
                grid_template_rows: vec![GridTrack::auto()],
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
        });
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder.spawn(NodeBundle {
        background_color: BackgroundColor(color),
        ..default()
    });
}
