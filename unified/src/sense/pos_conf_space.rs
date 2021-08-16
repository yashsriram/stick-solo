use ggez::{Context, GameResult};
use nalgebra::Point2;

pub struct PosConfSpace {
    pub obstacles: Vec<Box<dyn crate::sense::DrawableObstacle>>,
}

impl PosConfSpace {
    pub fn does_intersect_with_line_segment(&self, p1: Point2<f32>, p2: Point2<f32>) -> bool {
        for obs in self.obstacles.iter() {
            if obs.does_intersect_with_line_segment(p1, p2) {
                return true;
            }
        }
        return false;
    }

    pub fn does_intersect_with_point(&self, point: Point2<f32>, margin: f32) -> bool {
        for obs in self.obstacles.iter() {
            if obs.does_intersect_with_point(point, margin) {
                return true;
            }
        }
        return false;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for obs in self.obstacles.iter() {
            obs.draw(ctx)?
        }
        Ok(())
    }
}
