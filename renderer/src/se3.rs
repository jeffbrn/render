use nalgebra::{Const, Matrix3, Matrix4, OPoint, Point3, Rotation3, Vector3, Vector4};
use std::ops::Mul;

pub struct SE3 {
    pub mat: Matrix4<f32>,
}

impl SE3 {
    pub fn new() -> Self {
        Self {
            mat: Matrix4::identity(),
        }
    }

    pub fn from_translation_and_rotation(
        translation: Vector3<f32>,
        rotation: Rotation3<f32>,
    ) -> Self {
        let mut mat = Matrix4::identity();
        mat.fixed_view_mut::<3, 3>(0, 0)
            .copy_from(&rotation.to_homogeneous().fixed_view::<3, 3>(0, 0));
        mat.fixed_view_mut::<3, 1>(0, 3).copy_from(&translation);
        Self { mat }
    }

    pub fn translation(&self) -> Vector3<f32> {
        self.mat.fixed_view::<3, 1>(0, 3).into()
    }

    pub fn rotation(&self) -> Rotation3<f32> {
        let r: Matrix3<f32> = self.mat.fixed_view::<3, 3>(0, 0).into();
        Rotation3::from_matrix_unchecked(r)
    }

    pub fn transform(&self, point: Point3<f32>) -> Point3<f32> {
        let q = Vector4::new(point.x, point.y, point.z, 1.0);
        let result = self.mat * q;
        Point3::new(result.x, result.y, result.z)
    }
}
