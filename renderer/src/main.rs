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
    // Initial settings
    let mut angles = (0.0, 0.0, 0.0); // Roll, pitch, yaw
    let position = Vector3::<f32>::new(0.0, 0.0, 30.0);
    let canvas_size: (u32, u32) = (1024, 800);
    let cam = Camera::new(canvas_size.0, canvas_size.1, 0.1, 30.0, 30.0);
    let mut bx = Shape::new();

    println!("Control keys:");
    println!("Up/Down arrows: Pitch, Left/Right arrows: Roll, Z/X: Yaw, Esc: Exit");
    enable_raw_mode().expect("Failed to enable raw mode");

    loop {
        // Update shape attitude and position
        let attitude = set_attitude(angles.0, angles.1, angles.2);

        // Update output image
        let t = SE3::from_translation_and_rotation(position, attitude);
        let output_dir: String = "../output/".to_owned();

        // Initialize
        let mut img = RgbImage::new(canvas_size.0, canvas_size.1);
        bx.set_transform(t);
        bx.draw(&cam, &mut img);

        // Save the image as a PNG file
        img.save(output_dir + "img.png")
            .expect("Failed to save image");

            if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    angles.1 += 5.0; // Increase pitch
                    if angles.1 >= 360.0 {
                        angles.1 -= 360.0; // Wrap around
                    }
                },
                KeyCode::Down => {
                    angles.1 -= 5.0; // Decrease pitch
                    if angles.1 < 0.0 {
                        angles.1 += 360.0; // Wrap around
                    }
                },
                KeyCode::Left => {
                    angles.0 += 5.0; // Increase roll
                    if angles.0 >= 360.0 {
                        angles.0 -= 360.0; // Wrap around
                    }
                },
                KeyCode::Right => {
                    angles.0 -= 5.0; // Decrease roll
                    if angles.0 < 0.0 {
                        angles.0 += 360.0; // Wrap around
                    }
                },
                KeyCode::Char('z') => {
                    angles.2 += 5.0; // Increase yaw
                    if angles.2 >= 360.0 {
                        angles.2 -= 360.0; // Wrap around
                    }
                },
                KeyCode::Char('x') => {
                    angles.2 -= 5.0; // Decrease yaw
                    if angles.2 < 0.0 {
                        angles.2 += 360.0; // Wrap around
                    }
                },
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    disable_raw_mode().expect("Failed to disable raw mode");
}
