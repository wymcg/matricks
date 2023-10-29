/// Maps LEDs in a 2D matrix to a strip of LEDs
#[derive(Clone)]
pub(crate) struct MatrixMap {
    map: Vec<Vec<usize>>,
}

impl MatrixMap {
    /// Get the index in the LED strip of the pixel at matrix coordinate (x, y)
    ///
    /// # Arguments
    ///
    /// `x` - X-coordinate of the target LED
    /// `y` - Y-coordinate of the target LED
    ///
    pub(crate) fn get(&self, x: usize, y: usize) -> usize {
        self.map[y][x]
    }
}

#[derive(Clone)]
pub(crate) struct MatrixMapBuilder {
    width: usize,
    height: usize,
    serpentine: bool,
    vertical: bool,
    mirror_horizontal: bool,
    mirror_vertical: bool,
}

impl MatrixMapBuilder {
    /// Create a new MatrixMap builder
    ///
    /// # Arguments
    /// `width` - Width of the matrix, in number of LEDs
    /// `height` - Height of the matrix, in number of LEDs
    ///
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            serpentine: false,
            vertical: false,
            mirror_horizontal: false,
            mirror_vertical: false,
        }
    }

    /// Build a new MatrixMap
    pub(crate) fn build(&self) -> MatrixMap {
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

        // Mirror the matrix vertically if needed
        if self.mirror_vertical {
            map.reverse();
        }

        // Mirror the matrix horizontally if needed
        if self.mirror_horizontal {
            for y in 0..self.height {
                map[y].reverse();
            }
        }

        MatrixMap { map }
    }

    /// Specify that the matrix is serpentine
    pub(crate) fn serpentine(mut self) -> Self {
        self.serpentine = true;
        self
    }

    /// Specify that the matrix is vertically wired
    pub(crate) fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Mirror the matrix vertically
    pub(crate) fn mirror_vertically(mut self) -> Self {
        self.mirror_vertical = true;
        self
    }

    /// Mirror the matrix horizontally
    pub(crate) fn mirror_horizontally(mut self) -> Self {
        self.mirror_horizontal = true;
        self
    }
}
