use super::point::Point;

#[derive(Debug)]
pub struct BoundingBox {
    tl: Point,
    br: Point,
}

impl BoundingBox {
    pub fn new(tl: Point, br: Point) -> Self {
        Self { tl, br }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x() >= self.tl.x()
            && point.x() <= self.br.x()
            && point.y() >= self.tl.y()
            && point.y() <= self.br.y()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::point::Point;

    use super::BoundingBox;

    #[test]
    fn contains_should_return_false_if_point_outside_bounding_box() {
        let bounding_box = BoundingBox::new(Point::from((0, 0)), Point::from((2, 2)));

        assert!(!bounding_box.contains(Point::from((-1, -1))));
        assert!(!bounding_box.contains(Point::from((-1, 0))));
        assert!(!bounding_box.contains(Point::from((0, -1))));
        assert!(!bounding_box.contains(Point::from((3, 3))));
        assert!(!bounding_box.contains(Point::from((2, 3))));
        assert!(!bounding_box.contains(Point::from((3, 2))));
    }

    #[test]
    fn point_out_on_y_axis() {
        let bounding_box = BoundingBox::new(Point::from((0, 0)), Point::from((11, 11)));

        assert!(!bounding_box.contains(Point::from((9, -1))));
        assert!(!bounding_box.contains(Point::from((9, 12))));
    }

    #[test]
    fn point_out_on_x_axis() {
        let bounding_box = BoundingBox::new(Point::from((0, 0)), Point::from((11, 11)));

        assert!(!bounding_box.contains(Point::from((-1, 9))));
        assert!(!bounding_box.contains(Point::from((12, 9))));
    }
}
