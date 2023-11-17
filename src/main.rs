const COL_SIZE: usize = 10;
const ROW_SIZE: usize = 10;

const EMPTY_CHAR: &str = "░";
const FULL_CHAR: &str = "▓";

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
    println!("Hello, world!");

    let mut grid = [[false; COL_SIZE]; ROW_SIZE];

    // set live cells
    grid[5][5] = true;
    grid[9][0] = true;

    print_grid(&grid);
}

fn print_grid<const N: usize, const M: usize>(grid: &[[bool; N]; M]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { FULL_CHAR } else { EMPTY_CHAR })
        }
        println!();
    }
}
