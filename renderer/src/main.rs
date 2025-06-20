mod camera;
mod se3;
mod shape;

use camera::Camera;
use crossterm::event::{Event, KeyCode, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use image::RgbImage;
use nalgebra::{Rotation3, Vector3};
use se3::SE3;
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
    println!("Press an arrow key...");
    enable_raw_mode().expect("Failed to enable raw mode");

    loop {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => println!("Up arrow"),
                KeyCode::Down => println!("Down arrow"),
                KeyCode::Left => println!("Left arrow"),
                KeyCode::Right => println!("Right arrow"),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");

    // Settings
    let canvas_size: (u32, u32) = (1024, 800);
    let initial_attitude = set_attitude(15.0, 25.0, 10.0);
    let initial_posn = Vector3::<f32>::new(0.0, 0.0, 30.0);
    let t = SE3::from_translation_and_rotation(initial_posn, initial_attitude);
    let output_dir: String = "../output/".to_owned();

    // Initialize
    let mut img = RgbImage::new(canvas_size.0, canvas_size.1);
    let cam = Camera::new(canvas_size.0, canvas_size.1, 0.1, 30.0, 30.0);

    // println!("{:?}", cam);
    let mut bx = Shape::new();
    bx.set_transform(t);
    bx.draw(cam, &mut img);

    // Save the image as a PNG file
    img.save(output_dir + "img.png")
        .expect("Failed to save image");
}
