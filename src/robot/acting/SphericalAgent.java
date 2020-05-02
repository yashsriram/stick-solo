package robot.acting;

import math.Vec;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class SphericalAgent {
    public static float NEXT_MILESTONE_HINT_SIZE = 2f;
    public static float MILESTONE_REACHED_RADIUS = 2f;
    public static boolean DRAW_PATH = true;
    public static boolean DRAW_FUTURE_STATE = true;

    final PApplet parent;
    final Vec center;
    final float radius;
    final float speed;
    final Vec color;

    List<Vec> path = new ArrayList<>();
    int currentMilestone = 0;
    public boolean isPaused = false;

    public SphericalAgent(final PApplet parent,
                          Vec center,
                          float radius,
                          float speed,
                          Vec color) {
        this.parent = parent;
        this.center = new Vec(center);
        this.radius = radius;
        this.speed = speed;
        this.color = new Vec(color);
    }

    public void update(float dt) {
        if (isPaused) {
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // Reached next milestone?
            if (path.get(currentMilestone + 1).minus(center).norm() < MILESTONE_REACHED_RADIUS) {
                currentMilestone++;
                return;
            }
            // Move towards next milestone
            Vec velocityDir = path.get(currentMilestone + 1)
                    .minus(center)
                    .normalizeInPlace();
            Vec displacement = velocityDir.scaleInPlace(speed * dt);
            center.plusInPlace(displacement);
        }
    }

//    public void smoothUpdate(float dt) {
//        if (isPaused) {
//            return;
//        }
//        if (currentMilestone < path.size() - 1) {
//            // reached next milestone
//            if (path.get(currentMilestone + 1).minus(center).norm() < MILESTONE_REACHED_RADIUS) {
//                currentMilestone++;
//                return;
//            }
//            // next next milestone lookup
//            if (currentMilestone < path.size() - 2) {
//                boolean blocked = configurationSpace.doesEdgeIntersectSomeObstacle(path.get(currentMilestone + 2), center);
//                if (!blocked) {
//                    currentMilestone++;
//                }
//            }
//            // move towards next milestone
//            Vec3 velocityDir =
//                    path.get(currentMilestone + 1)
//                            .minus(center)
//                            .normalizeInPlace();
//            Vec3 displacement = velocityDir.scaleInPlace(speed * dt);
//            center.plusInPlace(displacement);
//            distanceCovered += displacement.norm();
//        }
//    }

    public void draw() {
        if (DRAW_PATH) {
            // path
            parent.stroke(color.get(0), color.get(1), color.get(2));
            for (int i = 0; i < path.size() - 1; i++) {
                Vec v1 = path.get(i);
                Vec v2 = path.get(i + 1);
                parent.line(v1.get(0), v1.get(1), v1.get(2), v2.get(0), v2.get(1), v2.get(2));
            }
            parent.noStroke();
        }
        // agent
        parent.pushMatrix();
        parent.fill(color.get(0), color.get(1), color.get(2));
        parent.translate(center.get(0), center.get(1), center.get(2));
        parent.sphere(radius);
        parent.popMatrix();
        if (DRAW_FUTURE_STATE) {
            // next milestone
            if (currentMilestone < path.size() - 1) {
                Vec nextMilestonePosition = path.get(currentMilestone + 1);
                parent.pushMatrix();
                parent.fill(1, 0, 0);
                parent.translate(nextMilestonePosition.get(0), nextMilestonePosition.get(1), nextMilestonePosition.get(2));
                parent.sphere(radius);
                parent.popMatrix();
            }
        }
    }

    public void setPath(Vec startPosition, List<Vec> newPath) {
        path = new ArrayList<>(newPath);
        currentMilestone = 0;
        center.set(startPosition);
    }

    public void stepForward() {
        if (path.size() == 0) {
            return;
        }
        center.set(path.get(currentMilestone));
        if (currentMilestone < path.size() - 1) {
            currentMilestone++;
        }
    }

    public void stepBackward() {
        if (path.size() == 0) {
            return;
        }
        center.set(path.get(currentMilestone));
        if (currentMilestone > 0) {
            currentMilestone--;
        }
    }

    public void reset(Vec startPosition) {
        center.set(startPosition);
        currentMilestone = 0;
    }
}
