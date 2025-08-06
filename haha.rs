use chaikin::*;
use macroquad::prelude::*;
// use std::time::*;

#[macroquad::main("chaikin")]
async fn main() {
    let mut original_points: Vec<Point> = Vec::new();
    let mut smooth_points: Vec<Point> = Vec::new();
    let mut animite = false;
    let mut last_time = 0.0;
    let mut steps = 0;
    let max_steps = 7;
    let delay = 0.5;
    let mut start_animate = false;

    while !is_key_pressed(KeyCode::Escape) {
        // Draw Original Points
        draw_points(&original_points);

        if is_key_pressed(KeyCode::Enter) && original_points.len() >= 2 && !start_animate {
            animite = true;
            smooth_points = original_points.clone();
            last_time = get_time();
        } else if is_key_pressed(KeyCode::C) {
            original_points.clear();
            smooth_points.clear();
            animite = false;
            steps = 0;
            start_animate = false;
        }

        if is_mouse_button_pressed(MouseButton::Left) && !animite {
            let (x, y) = mouse_position();
            let point = Point::new(x, y);
            original_points.push(point);
        }

        if animite && original_points.len() >= 2 {
            let now = get_time();
            start_animate = true;

            if now - last_time >= delay {
                if steps < max_steps {
                    smooth_points = chaikin_algo(&smooth_points);
                    steps += 1;
                } else {
                    smooth_points = original_points.clone();
                    steps = 0;
                }

                last_time = now;
            }

            draw_lines(&smooth_points);
        }

        next_frame().await;
    }
}

fn draw_points(original_points: &Vec<Point>) {
    for p in original_points {
        draw_circle_lines(p.x, p.y, 2.0, 1.0, WHITE);
    }
}

fn draw_lines(smooth_points: &Vec<Point>) {
    for i in 0..smooth_points.len() - 1 {
        let p1 = smooth_points[i];
        let p2 = smooth_points[i + 1];
        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, BLUE);
    }
}




#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn chaikin_algo(points: &Vec<Point>) -> Vec<Point> {
     if points.len() < 3 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();
    new_points.push(points[0]);
    
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        
        let q = Point::new(p1.x + 0.25 * (p2.x - p1.x), p1.y + 0.25 * (p2.y - p1.y));
        let r = Point::new(p1.x + 0.75 * (p2.x - p1.x), p1.y + 0.75 * (p2.y - p1.y));
        
        new_points.push(q);
        new_points.push(r);
    }
    
    new_points.push(points[points.len() - 1]);
    
    new_points
}
