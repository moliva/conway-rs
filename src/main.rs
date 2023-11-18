use conway_rs::{Grid, Stamp};

const COL_SIZE: usize = 100;
const ROW_SIZE: usize = 18;

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
    grid.stamp(Stamp::Oscillator, (8, 7));
    grid.stamp(Stamp::Glider, (0, 7));

    for i in 0.. {
        println!("tick {}", i);
        grid.print();
        grid.tick();

        use std::io::prelude::*;
        let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    }
}
