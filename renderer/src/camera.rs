use std::fmt::Debug;

use nalgebra::{Matrix3, Point2, Point3};

pub struct Camera {
    k: Matrix3<f32>,
    k_inv: Matrix3<f32>,
    dim_x: u16,
    dim_y: u16,
}

impl Camera {
    pub fn new(dim_x: u16, dim_y: u16, pixel_pitch: f32, fov_x: f32, fov_y: f32) -> Self {
        let k = Self::compute_intrinsic_matrix(pixel_pitch, dim_x, dim_y, fov_x, fov_y);
        let k_inv = k.try_inverse().expect("Camera matrix is not invertible");
        Camera {
            k,
            k_inv,
            dim_x,
            dim_y,
        }
    }

    pub fn project(&self, point: Point3<f32>) -> Point2<f32> {
        let projected = self.k * point;
        Point2::new(projected.x / projected.z, projected.y / projected.z)
    }

    pub fn unproject(&self, point: Point2<f32>) -> Point3<f32> {
        let point_homogeneous = Point3::new(point.x, point.y, 1.0);
        let unprojected = self.k_inv * point_homogeneous;
        unprojected
    }

    fn compute_intrinsic_matrix(
        pixel_pitch: f32,
        image_width: u16,
        image_height: u16,
        fov_x: f32,
        fov_y: f32,
    ) -> Matrix3<f32> {
        // Compute sensor size
        let sensor_width = image_width as f32 * pixel_pitch;
        let sensor_height = image_height as f32 * pixel_pitch;
        println!("Sensor size: {}mm x {}mm", sensor_width, sensor_height);
        println!(
            "FOV: {} x {}",
            (2.0 * (fov_x.to_radians() / 2.0).tan()),
            (2.0 * (fov_y.to_radians() / 2.0).tan())
        );

        // Compute focal lengths in pixels
        let fx = sensor_width / (2.0 * (fov_x.to_radians() / 2.0).tan());
        let fy = sensor_height / (2.0 * (fov_y.to_radians() / 2.0).tan());

        // Principal point (image center)
        let cx = image_width as f32 / 2.0;
        let cy = image_height as f32 / 2.0;

        // Construct the intrinsic matrix
        Matrix3::new(fx, 0.0, cx, 0.0, fy, cy, 0.0, 0.0, 1.0)
    }
}

impl Debug for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Camera")
            .field("k", &self.k)
            .field("dim_x", &self.dim_x)
            .field("dim_y", &self.dim_y)
            .finish()
    }
}
