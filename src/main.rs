use bevy::{diagnostic::LogDiagnosticsPlugin, input::common_conditions::*, prelude::*};

use components::{
    GameTimer, SelectedStamp, SimmetryResource, StampResource, COL_SIZE, ROW_SIZE, TICK_SECONDS,
};
use conway_rs::{Grid, Simmetry, Stamp};
use systems::{
    button_system, handle_click, menu_action, pause_system, setting_button, spawn_camera,
    spawn_grid, tickity,
};

mod components;
mod systems;

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
    grid.stamp(Stamp::Point, (9, 0), Simmetry::None);
    grid.stamp(Stamp::Block, (4, 4), Simmetry::None);
    grid.stamp(Stamp::Blinker, (8, 7), Simmetry::None);
    grid.stamp(Stamp::Glider, (0, 7), Simmetry::None);
    grid.stamp(Stamp::BeeHive, (4, 15), Simmetry::None);
    grid.stamp(Stamp::Tub, (10, 15), Simmetry::None);

    App::new()
        .insert_resource(grid)
        .insert_resource(SelectedStamp {
            stamp: Stamp::Point,
            simmetry: Simmetry::None,
        })
        .insert_resource(StampResource::Point)
        .insert_resource(SimmetryResource::None)
        .insert_resource(GameTimer(Timer::from_seconds(
            TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LogDiagnosticsPlugin::default())
        // .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                menu_action,
                handle_click.run_if(input_just_released(MouseButton::Left)),
                pause_system,
                setting_button::<StampResource>,
                setting_button::<SimmetryResource>,
                button_system,
                spawn_grid,
                tickity,
            ),
        )
        // Adds the plugins for each state
        // .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}
