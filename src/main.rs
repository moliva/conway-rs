use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};

use conway_rs::{Grid, Stamp};

const COL_SIZE: usize = 85;
const ROW_SIZE: usize = 40;

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
        // Spawn a 5 seconds timer to trigger going back to the menu
        .insert_resource(GameTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LogDiagnosticsPlugin::default())
        // .add_state::<GameState>()
        // Insert as resource the initial value for the settings resources
        // .insert_resource(TeamSize(3))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (spawn_grid, tickity)
        // Adds the plugins for each state
        // .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}

const ALIVE_COLOR: Color = Color::ANTIQUE_WHITE;
const DEAD_COLOR: Color = Color::DARK_GRAY;

const CELL_SIZE: f32 = 15.;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn tickity(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>,
) {
    if timer.tick(time.delta()).finished() {
        grid.tick();
    }
}

fn spawn_grid(mut commands: Commands, grid: ResMut<Grid<COL_SIZE, ROW_SIZE>>) {
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety it's parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of it's contents
                //   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content()],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of it's contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                grid_template_rows: vec![GridTrack::auto()],
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        // Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        // As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        // Use grid layout for this node
                        display: Display::Grid,
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::px(COL_SIZE, CELL_SIZE),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::px(ROW_SIZE as u16, CELL_SIZE),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    // let colors = [
                    //     Color::ORANGE,
                    //     Color::BISQUE,
                    //     Color::BLUE,
                    //     Color::CRIMSON,
                    //     Color::CYAN,
                    //     Color::ORANGE_RED,
                    //     Color::DARK_GREEN,
                    //     Color::FUCHSIA,
                    //     Color::TEAL,
                    //     Color::ALICE_BLUE,
                    //     Color::CRIMSON,
                    //     Color::ANTIQUE_WHITE,
                    //     Color::YELLOW,
                    //     Color::PINK,
                    //     Color::YELLOW_GREEN,
                    //     Color::SALMON,
                    // ];
                    //
                    // let mut color_it = colors.iter().cycle();
                    //
                    // let total = COL_SIZE * ROW_SIZE;
                    // for i in 0..total {
                    //     if total - 1 != i {
                    //         item_rect(builder, *color_it.next().unwrap());
                    //     }
                    // }

                    for row in grid.grid.iter() {
                        for &cell in row.iter() {
                            item_rect(builder, if cell { ALIVE_COLOR } else { DEAD_COLOR });
                        }
                    }
                });
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder.spawn(NodeBundle {
        background_color: BackgroundColor(color),
        ..default()
    });
}
