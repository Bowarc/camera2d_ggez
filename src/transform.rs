use ggez::{
    glam::Mat4,
    graphics::{self, DrawParam},
};
use math::{Point, Vec2};

#[derive(Clone, Copy)]
pub struct Transform {
    pub dest: Point,
    pub rotation: f64,
    pub scale: Vec2,
    pub offset: Point,
}

impl Transform {
    pub fn to_matrix(&self) -> Mat4 {
        let offset = self.offset / self.scale;

        let (sinr, cosr) = self.rotation.sin_cos();
        let m00 = cosr * self.scale.x;
        let m01 = -sinr * self.scale.y;
        let m10 = sinr * self.scale.x;
        let m11 = cosr * self.scale.y;
        let m03 = offset.x * (-m00) - offset.y * m01 + self.dest.x;
        let m13 = offset.y * (-m11) - offset.x * m10 + self.dest.y;

        Mat4::from_cols_array(&[
            m00 as f32, m01 as f32, 0.0, m03 as f32, //
            m10 as f32, m11 as f32, 0.0, m13 as f32, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ])
        .transpose()
    }

    pub fn apply_matrix(&self, parent_matrix: &Mat4) -> Mat4 {
        parent_matrix.mul_mat4(&self.to_matrix())
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            dest: Point::ZERO,
            rotation: 0.,
            scale: Vec2::ONE,
            offset: Point::ZERO,
        }
    }
}

impl From<Transform> for DrawParam {
    fn from(value: Transform) -> Self {
        DrawParam::default().transform(value.to_matrix())
    }
}

impl From<graphics::Transform> for Transform {
    fn from(value: graphics::Transform) -> Self {
        match value {
            graphics::Transform::Values {
                dest,
                rotation,
                scale,
                offset,
            } => Transform {
                dest: dest.into(),
                rotation: rotation as f64,
                scale: scale.into(),
                offset: offset.into(),
            },
            graphics::Transform::Matrix(_) => {
                panic!("Cannot convert ggez::Transform to crate::Transform")
            }
        }
    }
}
