use super::Camera;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_cross_mut;
use nalgebra::{Point2, Point3, Vector3};

pub struct Shape {
    points: Vec<Point3<f32>>,
    clr1: Rgb<u8>,
    lines: Vec<(usize, usize)>,
    clr2: Rgb<u8>,
}

impl Shape {
    pub fn new() -> Self {
        let mut retval = Self {
            points: vec![],
            clr1: Rgb([255, 255, 255]),
            lines: vec![],
            clr2: Rgb([128, 128, 128]),
        };
        retval.points.push(Point3::new(-10.0, -10.0, -10.0));
        retval.points.push(Point3::new(10.0, -10.0, -10.0));
        retval.points.push(Point3::new(10.0, 10.0, -10.0));
        retval.points.push(Point3::new(-10.0, 10.0, -10.0));
        retval.points.push(Point3::new(-10.0, -10.0, 10.0));
        retval.points.push(Point3::new(10.0, -10.0, 10.0));
        retval.points.push(Point3::new(10.0, 10.0, 10.0));
        retval.points.push(Point3::new(-10.0, 10.0, 10.0));
        retval.lines.push((0, 1));
        retval.lines.push((1, 2));
        retval.lines.push((2, 3));
        retval.lines.push((3, 0));
        retval.lines.push((4, 5));
        retval.lines.push((5, 6));
        retval.lines.push((6, 7));
        retval.lines.push((7, 4));

        retval.lines.push((0, 4));
        retval.lines.push((1, 5));
        retval.lines.push((2, 6));
        retval.lines.push((3, 7));
        retval
    }

    pub fn transform(&mut self, translation: Vector3<f32>, rotation: nalgebra::Rotation3<f32>) {
        for i in 0..self.points.len() {
            let p = self.points[i];
            let rotated = rotation * p.coords;
            self.points[i] = (rotated + translation).into(); //Point3::new(rotated.x + translation.x, rotated.y + translation.y, rotated.z + translation.z);
        }
    }
    pub fn draw(&self, cam: Camera, canvas: &mut RgbImage) {
        let mut list: Vec<Point2<f32>> = vec![];
        for i in 0..self.points.len() {
            let pnt = cam.project(self.points[i]);
            list.push(pnt);
            //            println!("Drawing point {}: {:?} --> ({}, {})", i, p, pnt.x, pnt.y);
            draw_cross_mut(canvas, self.clr1, pnt.x as i32, pnt.y as i32);
        }
        for i in 0..self.lines.len() {
            let line = &self.lines[i];
            let p1 = list[line.0];
            let p2 = list[line.1];
            // Draw the line between p1 and p2
            imageproc::drawing::draw_antialiased_line_segment_mut(
                canvas,
                (p1.x as i32, p1.y as i32),
                (p2.x as i32, p2.y as i32),
                self.clr2,
                |_line_color, _orig_color, _weight| _line_color,
            );
        }
    }
}
