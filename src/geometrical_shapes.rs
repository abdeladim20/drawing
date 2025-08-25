use raster::{Color, Image};
use rand::*;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        Color::white()
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Clone, Copy)]
pub struct Point (
    pub i32,
    pub i32,
);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point (
            x,
            y,
        )
    }

    pub fn random(max_x: i32, max_y: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(0..=max_x),
            rng.gen_range(0..=max_y),
        )
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color())
    }
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(point1: Point, point2: Point) -> Line{
        Line {
            start: point1, 
            end: point2,
        }
    }
    pub fn random(max_x: i32, max_y: i32) -> Self {
        Line {
            start: Point::random(max_x, max_y),
            end: Point::random(max_x, max_y),
        }
    }
}


impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;

        let steps = dx.abs().max(dy.abs());

        let x_increment = dx as f64/ steps as f64;
        let y_increment = dy as f64/ steps as f64;

        let mut current_x = self.start.0 as f64;
        let mut current_y = self.start.1 as f64;

        for _i in 0..=(steps as usize) {
            image.display(current_x as i32, current_y as i32, self.color());
            current_x += x_increment;
            current_y += y_increment;
        }
    }
}

pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

impl Triangle {
    pub fn new(point1: &Point, point2: &Point,point3: &Point) -> Self {
        Triangle {
            point1: *point1,
            point2: *point2,
            point3: *point3,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.point1, self.point2).draw(image);
        Line::new(self.point2, self.point3).draw(image);
        Line::new(self.point3, self.point1).draw(image);
    }
}

pub struct Rectangle {
    pub point1: Point,
    pub point2: Point,
}

impl Rectangle {
    pub fn new(point1: &Point, point2: &Point) -> Self {
        Rectangle {
            point1: *point1,
            point2: *point2,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let point3 = Point(self.point1.0, self.point2.1);
        let point4 = Point(self.point2.0, self.point1.1);
        Line::new(self.point1, point3).draw(image);
        Line::new(point3, self.point2).draw(image);
        Line::new(self.point2, point4).draw(image);
        Line::new(point4, self.point1).draw(image);
    }
}


#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Circle {
        Circle {
            center: Point(x, y),
            radius,                                 
        }
    }
    pub fn random(limit_x: i32, limit_y: i32) -> Circle {
        let mut rng = rand::thread_rng();

        Circle {
            center: Point::random(limit_x, limit_y),
            radius: rng.gen_range(9..=500),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut raster::Image) {
        let mut x = 0;
        let mut y = self.radius;
        let mut d: i64 = 3 - 2 * self.radius as i64;
        let color = self.color();

        while x <= y {
            let cx = self.center.0;
            let cy = self.center.1;

            image.display(cx + x, cy + y, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx + x, cy - y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx - y, cy - x, color.clone());

            if d < 0 {
                d += 4 * x as i64 + 6;
            } else {
                d += 4 * (x - y) as i64 + 10;
                y -= 1;
            }
            x += 1;
        }
    }


    fn color(&self) -> Color {
        let mut rng = thread_rng();
        Color::rgb(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    }

}
