use std::ops;
use std::cmp;

pub enum DirectionCompass {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn add(&self, x: i32, y: i32) -> Point {
        return Point::new(self.x + x, self.y + y);
    }

    fn within_points(&self, a: Point, b: Point) -> bool {
        let min_x = cmp::min(a.x, b.x);
        let max_x = cmp::max(a.x, b.x);
        let min_y = cmp::min(a.y, b.y);
        let max_y = cmp::max(a.y, b.y);

        return self.x >= min_x && self.x <= max_x && self.y >= min_y && self.y <= max_y;
    }

    fn manhattan(&self, other: Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn add_direction(&self, direction: DirectionCompass) -> Point {
        return match direction {
            DirectionCompass::N => self.add(0, -1),
            DirectionCompass::E => self.add(1, 0),
            DirectionCompass::S => self.add(0, 1),
            DirectionCompass::W => self.add(-1, 0)
        }
    }

    fn rotate90(&self) -> Point {
        Point::new(-self.y, self.x)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        return Point::new(self.x + _rhs.x, self.y + _rhs.y);
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        return Point::new(self.x - _rhs.x, self.y - _rhs.y);
    }
}

#[cfg(test)]
mod tests {
    fn p(x: i32, y:i32) -> super::Point {
        return super::Point::new(x, y);
    }

    #[test]
    fn point_add_sub() {
        let p0 = super::Point::origin();

        assert_eq!(0, p0.x);
        assert_eq!(0, p0.y);

        let p1 = p(1, -1);

        assert_eq!(1, p1.x);
        assert_eq!(-1, p1.y);

        let p2 = p1.add(1, 2);

        assert_eq!(2, p2.x);
        assert_eq!(1, p2.y);

        let p2 = p2 + p2;

        assert_eq!(4, p2.x);
        assert_eq!(2, p2.y);

        let p2 = p2 - p1;

        assert_eq!(3, p2.x);
        assert_eq!(3, p2.y);
    }

    #[test]
    fn point_add_dir() {
        let p00 = super::Point::origin();

        assert_eq!(p(0, -1), p00.add_direction(super::DirectionCompass::N));
        assert_eq!(p(1, 0), p00.add_direction(super::DirectionCompass::E));
        assert_eq!(p(0, 1), p00.add_direction(super::DirectionCompass::S));
        assert_eq!(p(-1, 0), p00.add_direction(super::DirectionCompass::W));
    }

    #[test]
    fn point_manhattan() {
        let p22 = p(2, 2);

        assert_eq!(4, p22.manhattan(super::Point::origin()));
    }

    #[test]
    fn point_within() {
        let p00 = super::Point::origin();
        let p22 = p(2, 2);
        let p44 = p(4, 4);
        let p55 = p(4, 4);

        assert_eq!(true, p22.within_points(p00, p44));
        assert_eq!(true, p22.within_points(p00, p22));
        assert_eq!(true, p22.within_points(p22, p44));
        assert_eq!(true, p22.within_points(p44, p00));

        assert_eq!(false, p22.within_points(p44, p55));
    }

    #[test]
    fn point_rotate() {
        let start = p(10, 0);

        let p1 = start.rotate90();
        assert_eq!(p(0, 10), p1);

        let p1 = p1.rotate90();
        assert_eq!(p(-10, 0), p1);

        let p1 = p1.rotate90();
        assert_eq!(p(0, -10), p1);

        let p1 = p1.rotate90();
        assert_eq!(start, p1);
    }
}