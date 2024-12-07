pub(crate) struct Matrix<T>(Vec<Vec<T>>);

impl<T: Default + Clone> Matrix<T> {
    pub(crate) fn n_columns(&self) -> usize {
        self.0.first().map(|f| f.len()).unwrap()
    }

    pub(crate) fn n_rows(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            top_left: (0, 0),
            bottom_right: (self.n_columns() - 1, self.n_rows() - 1),
        }
    }

    pub(crate) fn get(&self, x: usize, y: usize) -> T {
        self.0
            .get(y)
            .and_then(|line| line.get(x))
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn pairs(&self) -> MatrixCoordinates<T> {
        MatrixCoordinates {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    pub(crate) fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    pub(crate) fn iter_with_pos(&self) -> MatrixIteratorWithPos<T> {
        MatrixIteratorWithPos {
            matrix: self,
            row: 0,
            col: 0,
        }
    }

    pub(crate) fn find_position_by(&self, pred: fn(T) -> bool) -> Option<(usize, usize)> {
        for (x, y) in self.pairs() {
            let value = self.get(x, y);

            if pred(value) {
                return Some((x, y));
            }
        }

        None
    }
}

impl<T: Default + Clone> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

pub(crate) struct MatrixCoordinates<'a, T: Default + Clone> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Default + Clone> Iterator for MatrixCoordinates<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.matrix.n_rows() {
            return None;
        }

        let result = (self.col, self.row);

        self.col += 1;

        if self.col >= self.matrix.n_columns() {
            self.col = 0;
            self.row += 1;
        }

        Some(result)
    }
}

pub(crate) struct MatrixIterator<'a, T: Default + Clone> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Default + Clone> Iterator for MatrixIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.matrix.n_rows() {
            return None;
        }

        let result = self.matrix.get(self.col, self.row);

        self.col += 1;

        if self.col > self.matrix.n_columns() {
            self.col = 0;
            self.row += 1;
        }

        Some(result)
    }
}

pub(crate) struct MatrixIteratorWithPos<'a, T: Default + Clone> {
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Default + Clone> Iterator for MatrixIteratorWithPos<'a, T> {
    type Item = (T, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.matrix.n_rows() {
            return None;
        }

        let result = (self.matrix.get(self.col, self.row), (self.col, self.row));

        self.col += 1;

        if self.col >= self.matrix.n_columns() {
            self.col = 0;
            self.row += 1;
        }

        Some(result)
    }
}

#[derive(Default)]
pub(crate) struct BoundingBox {
    pub(crate) top_left: (usize, usize),
    pub(crate) bottom_right: (usize, usize),
}

impl From<&str> for Matrix<String> {
    fn from(value: &str) -> Self {
        let mut nodes: Vec<Vec<String>> = Vec::new();

        let mut curr_line: Vec<String> = Vec::new();

        for c in value.chars() {
            if c == '\n' {
                nodes.push(curr_line);
                curr_line = Vec::new();
            } else {
                curr_line.push(c.to_string());
            }
        }

        nodes.push(curr_line);

        Self::from(nodes)
    }
}
