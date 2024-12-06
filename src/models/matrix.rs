pub(crate) struct Matrix<T>(Vec<Vec<T>>);

impl<T: Default + Clone> Matrix<T> {
    pub(crate) fn n_columns(&self) -> usize {
        self.0.first().map(|f| f.len()).unwrap()
    }

    pub(crate) fn n_rows(&self) -> usize {
        self.0.len()
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
        if self.row >= self.matrix.n_rows() {
            return None;
        }

        let result = (self.row, self.col);

        self.col += 1;

        if self.col >= self.matrix.n_columns() {
            self.col = 0;
            self.row += 1;
        }

        Some(result)
    }
}
