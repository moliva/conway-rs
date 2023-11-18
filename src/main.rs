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
    grid[4][5] = true;
    grid[5][4] = true;
    grid[4][4] = true;

    grid[9][0] = true;

    grid[8][7] = true;
    grid[8][8] = true;
    grid[8][9] = true;

    let mut grid = Grid::new(grid);

    println!("tick 0");
    grid.print();

    grid.tick();

    println!("tick 1");
    grid.print();

    grid.tick();

    println!("tick 2");
    grid.print();
}

pub struct Grid<const N: usize, const M: usize> {
    grid: [[bool; N]; M],
    // rows: usize,
    // columns: usize,
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new(grid: [[bool; N]; M]) -> Self {
        Self {
            grid,
            // rows: N,
            // columns: M,
        }
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for &cell in row.iter() {
                print!("{}", if cell { FULL_CHAR } else { EMPTY_CHAR })
            }
            println!();
        }
    }

    /**
     * 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
     * 2. Any live cell with two or three live neighbours lives on to the next generation.
     * 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
     * 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
     */
    pub fn tick(&mut self) {
        let mut new_grid = [[false; N]; M];

        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let n = self.live_neighbors_around(i, j);

                if cell {
                    // * 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    // * 2. Any live cell with two or three live neighbours lives on to the next generation.
                    // * 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                    if n == 2 || n == 3 {
                        new_grid[i][j] = true;
                    }
                } else if n == 3 {
                    // * 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
                    new_grid[i][j] = true;
                }
            }
        }

        self.grid = new_grid;
    }

    fn live_neighbors_around(&self, i: usize, j: usize) -> usize {
        let mut live_neighbors = 0;

        if i > 0 {
            if j > 0 && self.grid[i - 1][j - 1] {
                live_neighbors += 1;
            }
            if self.grid[i - 1][j] {
                live_neighbors += 1;
            }
            if j < N - 1 && self.grid[i - 1][j + 1] {
                live_neighbors += 1;
            }
        }

        if j > 0 && self.grid[i][j - 1] {
            live_neighbors += 1;
        }
        if j < N - 1 && self.grid[i][j + 1] {
            live_neighbors += 1;
        }

        if i < M - 1 {
            if j > 0 && self.grid[i + 1][j - 1] {
                live_neighbors += 1;
            }
            if self.grid[i + 1][j] {
                live_neighbors += 1;
            }
            if j < N - 1 && self.grid[i + 1][j + 1] {
                live_neighbors += 1;
            }
        }

        live_neighbors
    }
}
