const EMPTY_CHAR: &str = "░";
const FULL_CHAR: &str = "▓";

#[derive(Debug)]
pub enum Stamp {
    Point,
    // still lifes
    Block,
    BeeHive,
    Loaf,
    Boat,
    Tub,
    // oscillators
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    PentaDecathlon,
    // spaceships
    Glider,
    LighWeightSpaceship,
    MiddleWeightSpaceship,
    HeavyWeightSpaceship,
}

pub struct Grid<const N: usize, const M: usize> {
    grid: [[bool; N]; M],
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn sized() -> Self {
        let grid = [[false; N]; M];

        Self::new(grid)
    }
    pub fn new(grid: [[bool; N]; M]) -> Self {
        Self { grid }
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

                // only set to true when conditions met for life to appear/persist
                if (cell && n == 2) || n == 3 {
                    new_grid[i][j] = true;
                }
            }
        }

        self.grid = new_grid;
    }

    fn mark_alive(&mut self, positions: &[(usize, usize)]) {
        for &(x, y) in positions {
            self.grid[x][y] = true;
        }
    }

    pub fn stamp(&mut self, stamp: Stamp, initial_position: (usize, usize)) {
        use Stamp::*;

        let (xi, yi) = initial_position;
        match stamp {
            Point => {
                self.mark_alive(&[initial_position]);
            }
            Block => self.mark_alive(&[
                initial_position,
                (xi, yi + 1),
                (xi + 1, yi),
                (xi + 1, yi + 1),
            ]),
            BeeHive => self.mark_alive(&[
                (xi + 1, yi),
                (xi, yi + 1),
                (xi + 2, yi + 1),
                (xi, yi + 2),
                (xi + 2, yi + 2),
                (xi + 1, yi + 3),
            ]),
            Loaf => todo!(),
            Boat => todo!(),
            Tub => self.mark_alive(&[
                (xi + 1, yi),
                (xi, yi + 1),
                (xi + 1, yi + 2),
                (xi + 2, yi + 1),
            ]),
            Blinker => self.mark_alive(&[initial_position, (xi, yi + 1), (xi, yi + 2)]),
            Toad => todo!(),
            Beacon => todo!(),
            Pulsar => todo!(),
            PentaDecathlon => todo!(),
            Glider => self.mark_alive(&[
                initial_position,
                (xi + 1, yi + 1),
                (xi + 1, yi + 2),
                (xi + 2, yi),
                (xi + 2, yi + 1),
            ]),
            LighWeightSpaceship => todo!(),
            MiddleWeightSpaceship => todo!(),
            HeavyWeightSpaceship => todo!(),
        }
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
