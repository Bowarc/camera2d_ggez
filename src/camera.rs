use ggez::{
    glam::{f64, Mat4, Vec3},
    graphics::DrawParam,
};
use math::{Point, Rect, Vec2};

use super::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub offset: Point,
    pub rotation: f64,
    pub scale: Vec2,
    pub position: Point,
    pub screen_size: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            offset: Point::ZERO,
            rotation: 0.,
            scale: Vec2::ONE,
            position: Point::ZERO,
            screen_size: Vec2::new(1920., 1080.),
        }
    }
}

impl Camera {
    pub fn new<P, V>(offset: P, rotation: f64, scale: V, position: P, screen_size: V) -> Self
    where
        P: Into<Point> + Copy,
        V: Into<Vec2>,
    {
        Camera {
            offset: offset.into(),
            rotation,
            scale: scale.into(),
            position: position.into(),
            screen_size: screen_size.into(),
        }
    }
    pub fn to_matrix(&self) -> Mat4 {
        let (sinr, cosr) = self.rotation.sin_cos();
        let m00 = cosr * self.scale.x;
        let m01 = -sinr * self.scale.y;
        let m10 = sinr * self.scale.x;
        let m11 = cosr * self.scale.y;
        let m03 = self.position.x * (-m00) - self.position.y * m01 + self.offset.x;
        let m13 = self.position.y * (-m11) - self.position.x * m10 + self.offset.y;

        Mat4::from_cols_array(&[
            m00 as f32, m01 as f32, 0.0, m03 as f32, //
            m10 as f32, m11 as f32, 0.0, m13 as f32, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ])
        .transpose()
    }

    pub fn apply_matrix<T>(&self, object: T) -> Mat4
    where
        T: Into<Transform>,
    {
        let object: Transform = object.into();

        self.to_matrix().mul_mat4(&object.to_matrix())
    }

    pub fn world_to_screen_coords<P>(&self, point: P) -> Point
    where
        P: Into<Point>,
    {
        let point: Point = point.into();
        let point = Vec3::new(point.x as f32, point.y as f32, 0.);
        let screen_point = self.to_matrix().transform_point3(point);

        Point::new(screen_point.x as f64, screen_point.y as f64)
    }

    pub fn screen_to_world_coords<P>(&self, point: P) -> Point
    where
        P: Into<Point>,
    {
        let inverse_matrix = self.to_matrix().inverse();
        let point: Point = point.into();
        let point = Vec3::new(point.x as f32, point.y as f32, 0.);
        let world_point = inverse_matrix.transform_point3(point);

        Point::new(world_point.x as f64, world_point.y as f64)
    }

    // Clockwise rotation
    pub fn world_view(&self) -> Rect {
        let topleft = self.screen_to_world_coords(0.);

        Rect::new(
            topleft,
            math::get_distance(
                &topleft,
                &self.screen_to_world_coords((self.screen_size.x, 0.)),
            ),
            0.,
        )
    }

    pub fn set_position<P>(&mut self, point: P)
    where
        P: Into<Point>,
    {
        self.position = point.into()
    }

    pub fn set_offset<P>(&mut self, point: P)
    where
        P: Into<Point>,
    {
        self.offset = point.into() * self.scale
    }

    pub fn move_by_world_coords<P>(&mut self, delta: P)
    where
        P: Into<Point>,
    {
        self.position -= delta.into()
    }

    pub fn move_by_screen_coords<P>(&mut self, delta: P)
    where
        P: Into<Point>,
    {
        self.position -= delta.into() / self.scale;
    }

    pub fn get_zoom(&self) -> Vec2 {
        self.scale
    }

    pub fn set_zoom<V>(&mut self, scale: V)
    where
        V: Into<Vec2>,
    {
        self.scale = scale.into();
    }

    pub fn zoom<V>(&mut self, factor: V)
    where
        V: Into<Vec2>,
    {
        self.scale *= factor.into();
    }

    pub fn zoom_center<V>(&mut self, factor: V)
    where
        V: Into<Vec2>,
    {
        let factor: Vec2 = factor.into();
        let screen_center = self.screen_size * 0.5;

        let world_center = self.screen_to_world_coords(screen_center);
        self.position.x = world_center.x - (world_center.x - self.position.x) / factor.x;
        self.position.y = world_center.y - (world_center.y - self.position.y) / factor.y;
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn zoom_at_screen_coords<P, V>(&mut self, point: P, factor: V)
    where
        P: Into<Point>,
        V: Into<Vec2>,
    {
        let point: Point = point.into();
        let factor: Vec2 = factor.into();
        let world_center = self.screen_to_world_coords(point);
        self.position.x = world_center.x - (world_center.x - self.position.x) / factor.x;
        self.position.y = world_center.y - (world_center.y - self.position.y) / factor.y;
        self.scale.x *= factor.x;
        self.scale.y *= factor.y;
    }

    pub fn rotate(&mut self, angle: f64) {
        self.rotation += angle;
    }

    pub fn set_rotation(&mut self, angle: f64) {
        self.rotation = angle;
    }

    pub fn screen_size(&self) -> math::Vec2 {
        self.screen_size
    }

    pub fn set_screen_size(&mut self, new_screen_size: impl Into<math::Vec2>) {
        self.screen_size = new_screen_size.into()
    }
}

impl From<Camera> for DrawParam {
    fn from(value: Camera) -> Self {
        DrawParam::default().transform(value.to_matrix())
    }
}
