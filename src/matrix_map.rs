#[derive(Clone)]
pub struct MatrixMap {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>
}

impl MatrixMap {
    pub fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }
}

#[derive(Clone)]
pub struct MatrixMapBuilder {
    width: usize,
    height: usize,
    serpentine: bool,
    vertical: bool,
}

impl MatrixMapBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            serpentine: false,
            vertical: false,
        }
    }

    pub fn build(&self) -> MatrixMap {
        // Determine the initial width and height of the map as we are constructing it
        let mut construct_width = self.width;
        let mut construct_height = self.height;

        // If this is a vertically wired matrix, construct a transpose map at first
        if self.vertical {
            (construct_width, construct_height) = (construct_height, construct_width);
        }

        // Fill in a normal matrix map
        let mut map: Vec<Vec<usize>> = vec![];
        for y in 0..construct_height {
            map.push(vec![]);
            for x in 0..construct_width {
                map[y].push((y * construct_width) + x);
            }
        }

        // If the matrix is serpentine, flip every other row
        if self.serpentine {
            for (y, row) in map.iter_mut().enumerate() {
                if y % 2 == 0 {
                    let reversed: Vec<usize> = row.clone().into_iter().rev().collect();
                    *row = reversed;
                }
            }
        }

        // If the matrix is vertical, transpose the matrix
        if self.vertical {
            let mut transposed_map: Vec<Vec<usize>> = vec![vec![0; self.width]; self.height];
            for y in 0..self.height {
                for x in 0..self.width {
                    transposed_map[y][x] = map[x][y];
                }
            }
            map = transposed_map
        }

        MatrixMap {
            width: self.width,
            height: self.height,
            map,
        }

    }

    pub fn serpentine(mut self) -> Self {
        self.serpentine = true;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}
