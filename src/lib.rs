use bevy::prelude::{Component, Resource};

const EMPTY_CHAR: &str = "░";
const FULL_CHAR: &str = "▓";

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Stamp {
    fn size(&self) -> (usize, usize) {
        use Stamp::*;

        match self {
            Point => (1, 1),
            Block => (2, 2),
            BeeHive => (3, 4),
            Loaf => todo!(),
            Boat => todo!(),
            Tub => (3, 3),
            Blinker => (1, 3),
            Toad => todo!(),
            Beacon => todo!(),
            Pulsar => todo!(),
            PentaDecathlon => todo!(),
            Glider => (3, 3),
            LighWeightSpaceship => todo!(),
            MiddleWeightSpaceship => todo!(),
            HeavyWeightSpaceship => todo!(),
        }
    }

    fn positions(&self) -> Vec<Position> {
        use Stamp::*;

        match self {
            Point => vec![(0, 0)],
            Block => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            BeeHive => vec![(1, 0), (0, 1), (2, 1), (0, 2), (2, 2), (1, 3)],
            Loaf => todo!(),
            Boat => todo!(),
            Tub => vec![(1, 0), (0, 1), (1, 2), (2, 1)],
            Blinker => vec![(0, 0), (0, 1), (0, 2)],
            Toad => todo!(),
            Beacon => todo!(),
            Pulsar => todo!(),
            PentaDecathlon => todo!(),
            Glider => vec![(0, 0), (1, 1), (1, 2), (2, 0), (2, 1)],
            LighWeightSpaceship => todo!(),
            MiddleWeightSpaceship => todo!(),
            HeavyWeightSpaceship => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Simmetry {
    None,
    X,
    Y,
    XY,
}

impl Simmetry {
    fn rotated(&self, stamp: Stamp) -> Vec<Position> {
        use Simmetry::*;

        let positions = stamp.positions();

        let (xl, yl) = stamp.size();
        let xl = xl - 1;
        let yl = yl - 1;

        match self {
            None => positions,
            X => positions.into_iter().map(|(x, y)| (xl - x, y)).collect(),
            Y => positions.into_iter().map(|(x, y)| (x, yl - y)).collect(),
            XY => positions
                .into_iter()
                .map(|(x, y)| (xl - x, yl - y))
                .collect(),
        }
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Grid<const C: usize, const R: usize> {
    pub grid: [[bool; C]; R],
}

impl<const C: usize, const R: usize> Grid<C, R> {
    pub fn sized() -> Self {
        let grid = [[false; C]; R];

        Self::new(grid)
    }
    pub fn new(grid: [[bool; C]; R]) -> Self {
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
        let mut new_grid = [[false; C]; R];

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

    pub fn stamp(&mut self, stamp: Stamp, (xi, yi): Position, rotation: Simmetry) {
        let positions = rotation
            .rotated(stamp)
            .into_iter()
            .map(|(x, y)| (x + xi, y + yi))
            .collect::<Vec<_>>();

        self.mark_alive(&positions[..])
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
            if j < C - 1 && self.grid[i - 1][j + 1] {
                live_neighbors += 1;
            }
        }

        if j > 0 && self.grid[i][j - 1] {
            live_neighbors += 1;
        }
        if j < C - 1 && self.grid[i][j + 1] {
            live_neighbors += 1;
        }

        if i < R - 1 {
            if j > 0 && self.grid[i + 1][j - 1] {
                live_neighbors += 1;
            }
            if self.grid[i + 1][j] {
                live_neighbors += 1;
            }
            if j < C - 1 && self.grid[i + 1][j + 1] {
                live_neighbors += 1;
            }
        }

        live_neighbors
    }
}
