#[derive(Clone)]
pub struct MatrixMap {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>
}

impl MatrixMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map: Vec<Vec<usize>> = vec![];
        for y in 0..height {
            map.push(vec![]);
            for x in 0..width {
                let strip_index = (y * width) + x;
                map[y].push(strip_index);
            }
        }

        Self {
            width,
            height,
            map
        }
    }

    pub fn serpentine(&self) -> Self {
        let mut map = self.map.clone();
        for (y, row) in map.iter_mut().enumerate() {
            if y % 2 == 1 {
                row.reverse();
            }
        }

        Self {
            width: self.width,
            height: self.height,
            map
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }
}
