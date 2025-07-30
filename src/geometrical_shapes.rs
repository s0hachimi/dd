use raster::{ Color };
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub point1: Point,
    pub point2: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: u32,
}

pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

pub struct Pentagon {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
    pub point4: Point,
    pub point5: Point,
}

pub struct Cube {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
    pub point4: Point,
    pub point5: Point,
    pub point6: Point,  
    pub point7: Point,
    pub point8: Point,
}

pub trait Drawable {
    fn draw<T: Displayable>(&self, image: &mut T);
    fn color() -> Color {
        Color::rgb(
            (rand::random::<u8>() % 127) + 128,
            (rand::random::<u8>() % 127) + 128,
            (rand::random::<u8>() % 127) + 128
        )
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::random::<u32>() % (width as u32);
        let y = rand::random::<u32>() % (height as u32);
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl Drawable for Point {
    fn draw<T: Displayable>(&self, image: &mut T) {
        image.display(self.x as i32, self.y as i32, Self::color());
    }
}

impl Line {
    pub fn new(point1: Point, point2: Point) -> Self {
        Self { point1, point2 }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let point1 = Point::random(width, height);
        let point2 = Point::random(width, height);
        Self::new(point1, point2)
    }
}

fn draw_line<T: Displayable>(image: &mut T, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let x0 = x0 as f32;
    let y0 = y0 as f32;
    let x1 = x1 as f32;
    let y1 = y1 as f32;

    let dx = x1 - x0;
    let dy = y1 - y0;
    
    let steps = dx.abs().max(dy.abs()) as i32;

    let x_inc = dx / (steps as f32);
    let y_inc = dy / (steps as f32);

    let mut x = x0;
    let mut y = y0;

    for _ in 0..=steps {
        image.display(x.round() as i32, y.round() as i32, color.clone());
        x += x_inc;
        y += y_inc;
    }
}

impl Drawable for Line {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = Self::color();
        draw_line(image, self.point1.x, self.point1.y, self.point2.x, self.point2.y, color);
    }
}

fn trigonometric_circle(cx: i32, cy: i32, r: u32, step: f32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let mut theta = 0.0;

    while theta <= 2.0 * PI {
        let x = (cx as f32) + (r as f32) * theta.cos();
        let y = (cy as f32) + (r as f32) * theta.sin();
        points.push((x.round() as i32, y.round() as i32));
        theta += step;
    }
    points
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Circle {
        let center = Point {
            x: (rand::random::<u32>() % (width as u32)) as i32,
            y: (rand::random::<u32>() % (height as u32)) as i32,
        };

        let radius = rand::random::<u32>() % ((width / 4) as u32);
        Circle { center, radius }
    }
}

impl Drawable for Circle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let rgb = Self::color();
        let step = 0.1 / (self.radius as f32);
        let circle_points = trigonometric_circle(self.center.x, self.center.y, self.radius, step);
        for (x, y) in circle_points {
            let _ = image.display(x, y, rgb.clone());
        }
    }
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            top_left: Point {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            bottom_right: Point {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        }
    }
}

impl Drawable for Rectangle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = Self::color();
        let x1 = self.top_left.x;
        let y1 = self.top_left.y;
        let x2 = self.bottom_right.x;
        let y2 = self.bottom_right.y;

        for x in x1..=x2 {
            image.display(x, y1, color.clone());
            image.display(x, y2, color.clone());
        }

        for y in y1..=y2 {
            image.display(x1, y, color.clone());
            image.display(x2, y, color.clone());
        }
    }
}

impl Triangle {
    pub fn new(point1: &Point, point2: &Point, point3: &Point) -> Self {
        Self {
            point1: point1.clone(),
            point2: point2.clone(),
            point3: point3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = Self::color();
        draw_line(image, self.point1.x, self.point1.y, self.point2.x, self.point2.y, color.clone());
        draw_line(image, self.point2.x, self.point2.y, self.point3.x, self.point3.y, color.clone());
        draw_line(image, self.point3.x, self.point3.y, self.point1.x, self.point1.y, color.clone());
    }
}

impl Pentagon {
    pub fn new(
        point1: &Point,
        point2: &Point,
        point3: &Point,
        point4: &Point,
        point5: &Point
    ) -> Self {
        Self {
            point1: point1.clone(),
            point2: point2.clone(),
            point3: point3.clone(),
            point4: point4.clone(),
            point5: point5.clone(),
        }
    }
}

impl Drawable for Pentagon {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = Self::color();
        draw_line(image, self.point1.x, self.point1.y, self.point2.x, self.point2.y, color.clone());
        draw_line(image, self.point2.x, self.point2.y, self.point3.x, self.point3.y, color.clone());
        draw_line(image, self.point3.x, self.point3.y, self.point4.x, self.point4.y, color.clone());
        draw_line(image, self.point4.x, self.point4.y, self.point5.x, self.point5.y, color.clone());
        draw_line(image, self.point5.x, self.point5.y, self.point1.x, self.point1.y, color.clone());
    }
}

impl Cube {
    pub fn new(
        point1: &Point,
        point2: &Point,
        point3: &Point,
        point4: &Point,
        point5: &Point,
        point6: &Point,
        point7: &Point,
        point8: &Point
    ) -> Self {
        Self {
            point1: point1.clone(),
            point2: point2.clone(),
            point3: point3.clone(),
            point4: point4.clone(),
            point5: point5.clone(),
            point6: point6.clone(),
            point7: point7.clone(),
            point8: point8.clone(),
        }
    }
}

impl Drawable for Cube {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = Self::color();
        draw_line(image, self.point1.x, self.point1.y, self.point2.x, self.point2.y, color.clone());
        draw_line(image, self.point2.x, self.point2.y, self.point3.x, self.point3.y, color.clone());
        draw_line(image, self.point3.x, self.point3.y, self.point4.x, self.point4.y, color.clone());
        draw_line(image, self.point4.x, self.point4.y, self.point1.x, self.point1.y, color.clone());

        draw_line(image, self.point5.x, self.point5.y, self.point6.x, self.point6.y, color.clone());
        draw_line(image, self.point6.x, self.point6.y, self.point7.x, self.point7.y, color.clone());
        draw_line(image, self.point7.x, self.point7.y, self.point8.x, self.point8.y, color.clone());
        draw_line(image, self.point8.x, self.point8.y, self.point5.x, self.point5.y, color.clone());

        draw_line(image, self.point1.x, self.point1.y, self.point5.x, self.point5.y, color.clone());
        draw_line(image, self.point2.x, self.point2.y, self.point6.x, self.point6.y, color.clone());
        draw_line(image, self.point3.x, self.point3.y, self.point7.x, self.point7.y, color.clone());
        draw_line(image, self.point4.x, self.point4.y, self.point8.x, self.point8.y, color.clone());
    }
}
