use nalgebra::Point2;

pub mod obstacle;
pub mod pos_conf_space;

pub trait Obstacle {
    fn does_intersect_with_line_segment(&self, p1: Point2<f32>, p2: Point2<f32>) -> bool;
    fn does_intersect_with_point(&self, point: Point2<f32>, margin: f32) -> bool;
}

pub trait DrawableObstacle: Obstacle + crate::Draw {}

