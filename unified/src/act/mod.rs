use nalgebra::Point2;

pub struct CircularAgent {
    center: Point2<f32>,
    radius: f32,
    speed: f32,
    color: [f32; 4],
}

impl CircularAgent {
    pub fn new(center: Point2<f32>, radius: f32, speed: f32, color: [f32; 4]) -> Self {
        Self {
            center,
            radius,
            speed,
            color,
        }
    }
}

// public class CircularAgent {
//     public static float MILESTONE_REACHED_SLACK = 2f;
//     public boolean isPaused = false;

//     private List<Milestone> path = new ArrayList<>();
//     private int currentMilestone = 0;

//     public void spawn(Vec startPosition, List<Milestone> newPath) {
//         center.set(startPosition);
//         path = new ArrayList<>(newPath);
//         currentMilestone = 0;
//     }

//     public void stepForward() {
//         if (path.size() == 0) {
//             return;
//         }
//         center.set(path.get(currentMilestone).position);
//         if (currentMilestone < path.size() - 1) {
//             currentMilestone++;
//         }
//     }

//     public void stepBackward() {
//         if (path.size() == 0) {
//             return;
//         }
//         center.set(path.get(currentMilestone).position);
//         if (currentMilestone > 0) {
//             currentMilestone--;
//         }
//     }

//     public void update(float dt) {
//         if (isPaused) {
//             return;
//         }
//         if (currentMilestone < path.size() - 1) {
//             // Reached next milestone?
//             if (path.get(currentMilestone + 1).position.minus(center).norm() < MILESTONE_REACHED_SLACK) {
//                 currentMilestone++;
//                 return;
//             }
//             // Move towards next milestone
//             Vec velocityDir = path.get(currentMilestone + 1)
//             		.position
//             		.minus(center)
//                     .normalizeInPlace();
//             Vec displacement = velocityDir.scaleInPlace(speed * dt);
//             center.plusInPlace(displacement);
//         }
//     }

//     public void draw() {
//         // path
//         parent.stroke(color.get(0), color.get(1), color.get(2));
//         for (int i = 0; i < path.size() - 1; i++) {
//             Vec v1 = path.get(i).position;
//             Vec v2 = path.get(i + 1).position;
//             parent.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
//         }
//         parent.noStroke();
//         // agent
//         parent.pushMatrix();
//         parent.fill(color.get(0), color.get(1), color.get(2));
//         parent.translate(0, center.get(1), center.get(0));
//         parent.sphere(radius);
//         parent.popMatrix();
//         // next milestone
//         if (currentMilestone < path.size() - 1) {
//             Vec nextMilestonePosition = path.get(currentMilestone + 1).position;
//             parent.pushMatrix();
//             parent.fill(1, 0, 0);
//             parent.translate(0, nextMilestonePosition.get(1), nextMilestonePosition.get(0));
//             parent.sphere(radius);
//             parent.popMatrix();
//         }
//     }
// }
