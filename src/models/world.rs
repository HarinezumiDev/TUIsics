#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: usize,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<Cell>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(None);
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.in_bounds(x, y) && self.cells[self.idx(x, y)].is_none()
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<Cell> {
        if self.in_bounds(x, y) {
            self.cells[self.idx(x, y)]
        } else {
            None
        }
    }

    pub fn spawn_at(&mut self, x: usize, y: usize, material: usize) -> bool {
        if x >= self.width || self.height == 0 {
            return false;
        }

        let start = y.min(self.height - 1);

        for yy in (0..=start).rev() {
            let idx = self.idx(x, yy);
            if self.cells[idx].is_none() {
                self.cells[idx] = Some(Cell { material });
                return true;
            }
        }

        for yy in start + 1..self.height {
            let idx = self.idx(x, yy);
            if self.cells[idx].is_none() {
                self.cells[idx] = Some(Cell { material });
                return true;
            }
        }

        false
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let ia = self.idx(a.0, a.1);
        let ib = self.idx(b.0, b.1);
        self.cells.swap(ia, ib);
    }
}