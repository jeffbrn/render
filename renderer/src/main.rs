/*
use nalgebra::{Rotation3, Vector3};

fn main() {
    let roll: f32 = 45.0;
    let roll_radians = roll.to_radians();
    println!("cos = {}, sin = {}", roll_radians.cos(), roll_radians.sin());
    let x_axis = Vector3::<f32>::z_axis();
    println!("Z Axis: {:?}", x_axis);
    let rot = Rotation3::<f32>::from_axis_angle(&x_axis, roll_radians);
    println!("Rotation: {:?}", rot);
    let p = Vector3::<f32>::new(1.0, 2.0, 3.0);
    let q = rot * p;
    println!("Transformed Point: {:?} --> {:?}", p, q);
}
*/

mod camera;
mod shape;

use camera::Camera;
use image::RgbImage;
use nalgebra::{Rotation3, Vector3};
use shape::Shape;

fn set_attitude(roll: f32, pitch: f32, yaw: f32) -> Rotation3<f32> {
    let roll_rad = roll.to_radians();
    let pitch_rad = pitch.to_radians();
    let yaw_rad = yaw.to_radians();

    let rot_x = Rotation3::from_axis_angle(&Vector3::x_axis(), pitch_rad);
    let rot_y = Rotation3::from_axis_angle(&Vector3::y_axis(), yaw_rad);
    let rot_z = Rotation3::from_axis_angle(&Vector3::z_axis(), roll_rad);

    rot_x * rot_y * rot_z
}

fn main() {
    let rot = set_attitude(15.0, 25.0, 15.0);
    // Create a blank image with white background
    let mut img = RgbImage::new(400, 300);
    let cam = Camera::new(400, 300, 0.1, 30.0, 30.0);
    println!("{:?}", cam);
    let mut bx = Shape::new();
    let posn = Vector3::<f32>::new(0.0, 0.0, 30.0);
    bx.transform(posn, rot);
    bx.draw(cam, &mut img);
    // clear(&mut img);
    // draw(&mut img);

    // Save the image as a PNG file
    img.save("output.png").expect("Failed to save image");

    println!("Image saved as output.png");
}

/*
fn _clear(canvas: &mut RgbImage) {
    for pixel in canvas.pixels_mut() {
        *pixel = Rgb([0, 0, 0]); // Clear to black
    }
}

fn _draw(canvas: &mut RgbImage) {
    // Define a rectangle
    // let rect = Rect::at(50, 50).of_size(200, 100);

    // Draw a filled rectangle with red color
    // draw_filled_rect_mut(canvas, rect, Rgb([255, 0, 0]));

    draw_cross_mut(canvas, Rgb([0, 255, 0]), 100, 100);
    let clr = Rgb([255, 0, 255]);
    draw_antialiased_line_segment_mut(
        canvas,
        (20, 20),
        (80, 80),
        clr,
        |_line_color, _orig_color, _weight| _line_color,
    );
}
*/
