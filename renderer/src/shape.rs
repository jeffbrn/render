use super::Camera;
use super::se3::SE3;
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_cross_mut;
use nalgebra::{Point2, Point3, Vector3};

pub struct Shape {
    points: Vec<Point3<f32>>,
    clr1: Rgb<u8>,
    lines: Vec<(usize, usize)>,
    clr2: Rgb<u8>,
    transform: SE3,
}

impl Shape {
    pub fn new() -> Self {
        let mut retval = Self {
            points: vec![],
            clr1: Rgb([255, 255, 255]),
            lines: vec![],
            clr2: Rgb([128, 128, 128]),
            transform: SE3::new(),
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

    pub fn set_transform(&mut self, t: SE3) {
        self.transform = t;
    }
    pub fn _transform(&mut self, translation: Vector3<f32>, rotation: nalgebra::Rotation3<f32>) {
        for i in 0..self.points.len() {
            let p = self.points[i];
            let rotated = rotation * p.coords;
            self.points[i] = (rotated + translation).into();
        }
    }
    pub fn transform_se3(&mut self, t: &SE3) {
        for i in 0..self.points.len() {
            let p = t.transform(self.points[i]);
            self.points[i] = p;
        }
    }
    fn project_points(&self, cam: &Camera) -> Vec<Point2<f32>> {
        self.points
            .iter()
            .map(|p| cam.project(self.transform.transform(*p)))
            .collect()
    }
    fn draw_shape(&self, canvas: &mut RgbImage, uvs: &[Point2<f32>]) {
        for i in 0..uvs.len() {
            draw_cross_mut(canvas, self.clr1, uvs[i].x as i32, uvs[i].y as i32);
        }
        for line in &self.lines {
            let p1 = uvs[line.0];
            let p2 = uvs[line.1];
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
    fn draw_legend(&self, canvas: &mut RgbImage) {
        let n = self.transform.mat;
        let line = format!(
            "{:#8.2},{:#8.2},{:#8.2},{:#8.2}",
            n[(0, 0)],
            n[(0, 1)],
            n[(0, 2)],
            n[(0, 3)]
        );
        let y = self.write_ln(canvas, &line, 10, 10);
        let line = format!(
            "{:#8.2},{:#8.2},{:#8.2},{:#8.2}",
            n[(1, 0)],
            n[(1, 1)],
            n[(1, 2)],
            n[(1, 3)]
        );
        let y = self.write_ln(canvas, &line, 10, y);
        let line = format!(
            "{:#8.2},{:#8.2},{:#8.2},{:#8.2}",
            n[(2, 0)],
            n[(2, 1)],
            n[(2, 2)],
            n[(2, 3)]
        );
        let _y = self.write_ln(canvas, &line, 10, y);
    }
    fn write_ln(&self, canvas: &mut RgbImage, text: &str, x: i32, y: i32) -> i32 {
        let font = FontRef::try_from_slice(include_bytes!("../arial.ttf")).unwrap();
        let txt_clr = Rgb([255, 255, 0]);
        let height = 16.2;
        let scale = PxScale {
            x: height * 1.0,
            y: height * 1.0,
        };
        imageproc::drawing::draw_text_mut(canvas, txt_clr, x, y, scale, &font, text);
        let (_w, h) = imageproc::drawing::text_size(scale, &font, text);
        let a = h as f32 * 1.2;
        (y as f32 + a) as i32
    }
    pub fn draw(&self, cam: Camera, canvas: &mut RgbImage) {
        let uvs = self.project_points(&cam);
        self.draw_shape(canvas, &uvs);
        self.draw_legend(canvas);
    }
}
