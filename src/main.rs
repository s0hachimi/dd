mod geometrical_shapes;
use geometrical_shapes as gs;

use raster::{ Color, Image };
use gs::{ Displayable, Drawable };

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800)
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    let pentagon = gs::Pentagon::new(
        &gs::Point::new(400, 200),
        &gs::Point::new(590, 338),
        &gs::Point::new(518, 562),
        &gs::Point::new(282, 562),
        &gs::Point::new(210, 338)
    );
    pentagon.draw(&mut image);

    let cube = gs::Cube::new(
        // Front face
        &gs::Point::new(500, 500),
        &gs::Point::new(700, 500),
        &gs::Point::new(700, 700),
        &gs::Point::new(500, 700),

        // Back face
        &gs::Point::new(550, 450),
        &gs::Point::new(750, 450),
        &gs::Point::new(750, 650),
        &gs::Point::new(550, 650)
    );
    cube.draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
