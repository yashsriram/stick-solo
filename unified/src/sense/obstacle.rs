use ggez::{
    graphics,
    graphics::{DrawMode, DrawParam, MeshBuilder},
    Context, GameResult,
};
use nalgebra::{Matrix2, Matrix2x1, Point2, Vector2};

pub struct CircleObstacle {
    center: Point2<f32>,
    radius: f32,
    color: [f32; 4],
}

impl CircleObstacle {
    pub fn new(center: Point2<f32>, radius: f32) -> Self {
        assert!(radius > 0.0);
        CircleObstacle {
            center,
            radius,
            color: [1.0, 0.0, 1.0, 1.0],
        }
    }
}

impl crate::Draw for CircleObstacle {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = MeshBuilder::new()
            .circle(
                DrawMode::stroke(1.0),
                self.center,
                self.radius,
                1.0,
                self.color.into(),
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}

impl crate::sense::Obstacle for CircleObstacle {
    fn does_intersect_with_line_segment(&self, p1: Point2<f32>, p2: Point2<f32>) -> bool {
        let pb_pa = p2 - p1;
        let pa_pc = p1 - self.center;
        let r = self.radius;
        let a = pb_pa.dot(&pb_pa);
        let c = pa_pc.dot(&pa_pc) - r * r;
        let b = 2.0 * pb_pa.dot(&pa_pc);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            // Intersection with line segment only possible iff at least one of the solutions lies in [0, 1]
            return (0.0 <= t1 && t1 <= 1.0) || (0.0 <= t2 && t2 <= 1.0);
        }
        // Discriminant < 0 => no intersection
        return false;
    }

    fn does_intersect_with_point(&self, point: Point2<f32>, margin: f32) -> bool {
        if (self.center - point).norm() <= (self.radius + margin) {
            return true;
        }
        return false;
    }
}

impl crate::sense::DrawableObstacle for CircleObstacle {}

pub struct LineSegmentObstacle {
    e1: Point2<f32>,
    e2: Point2<f32>,
    color: [f32; 4],
}

impl LineSegmentObstacle {
    pub fn new(e1: Point2<f32>, e2: Point2<f32>) -> Self {
        assert!((e1 - e2).norm() > 0.0);
        LineSegmentObstacle {
            e1,
            e2,
            color: [1.0, 0.0, 1.0, 1.0],
        }
    }
}

impl crate::Draw for LineSegmentObstacle {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = MeshBuilder::new()
            .line(&[self.e1, self.e2], 0.5, self.color.into())?
            .circle(DrawMode::fill(), self.e1, 5.0, 0.5, self.color.into())?
            .circle(DrawMode::fill(), self.e2, 5.0, 0.5, self.color.into())?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}

impl crate::sense::Obstacle for LineSegmentObstacle {
    fn does_intersect_with_line_segment(&self, p1: Point2<f32>, p2: Point2<f32>) -> bool {
        fn find_scale(original: &Vector2<f32>, scaled: &Vector2<f32>) -> f32 {
            let mut scale = 0.0;
            let mut num_non_zero_elements = 0usize;
            if original[0].abs() > 1e-4 {
                scale += scaled[0] / original[0];
                num_non_zero_elements += 1;
            }
            if original[1].abs() > 1e-4 {
                scale += scaled[1] / original[1];
                num_non_zero_elements += 1;
            }
            scale = scale / num_non_zero_elements as f32;
            scale
        }
        let (e1, e2) = (self.e1, self.e2);
        // mat_a
        let mat_a = Matrix2::new(
            p2[1] - p1[1],
            -(p2[0] - p1[0]),
            e2[1] - e1[1],
            -(e2[0] - e1[0]),
        );
        // b
        let b = Matrix2x1::new(p1[0] * p2[1] - p1[1] * p2[0], e1[0] * e2[1] - e1[1] * e2[0]);
        if mat_a.determinant().abs() < 1e-6 {
            let v1 = (p2 - e1).normalize();
            let v2 = (e2 - e1).normalize();
            if 1.0 - v1.dot(&v2).abs() < 1e-6 {
                // Coincident check if either p1 or p2 falls in b/w e1 and e2
                return ((e1 - p1).norm() + (e2 - p1).norm() - (e2 - e1).norm()).abs() < 1e-6
                    || ((e1 - p2).norm() + (e2 - p2).norm() - (e2 - e1).norm()).abs() < 1e-6;
            } else {
                // Parallel
                return false;
            }
        }
        // mat_ax = b
        let x = nalgebra::linalg::QR::new(mat_a).solve(&b).unwrap();
        // Is intersection b/w link end points?
        let end_length = (p2 - p1).norm();
        let end_unit_vec = (p2 - p1).normalize();
        let scaled_end_unit_vec = x - p1.coords;
        let t1 = find_scale(&end_unit_vec, &scaled_end_unit_vec);
        if t1 < 0.0 || t1 > end_length {
            return false;
        }
        // Is intersection b/w obs end points?
        let obs_length = (e2 - e1).norm();
        let obs_unit_vec = (e2 - e1).normalize();
        let scaled_obs_unit_vec = x - e1.coords;
        let t2 = find_scale(&obs_unit_vec, &scaled_obs_unit_vec);
        if t2 < 0.0 || t2 > obs_length {
            return false;
        }
        // All conditions are okay then return true
        return true;
    }

    fn does_intersect_with_point(&self, point: Point2<f32>, margin: f32) -> bool {
        let l2_sqr = (self.e1 - self.e2).norm_squared();
        let min_dist_from_line_segement = if l2_sqr == 0.0 {
            (point - self.e1).norm()
        } else {
            // Treats this as a line segment not a line
            let dir_vec = self.e2 - self.e1;
            let to_point = point - self.e1;
            let unclipped_t = to_point.dot(&dir_vec) / l2_sqr;
            let clipped_t = 0.0f32.max(unclipped_t).min(1.0f32);
            let nearest_point = self.e1 + (self.e2 - self.e1) * clipped_t;
            (point - nearest_point).norm()
        };
        min_dist_from_line_segement <= margin
    }
}

impl crate::sense::DrawableObstacle for LineSegmentObstacle {}
