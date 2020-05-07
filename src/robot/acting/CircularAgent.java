package robot.acting;

import math.Vec;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class CircularAgent {
    public static float MILESTONE_REACHED_SLACK = 2f;
    public boolean isPaused = false;

    private final PApplet parent;
    private final Vec center;
    private final float radius;
    private final float speed;
    private final Vec color;

    private List<Vec> path = new ArrayList<>();
    private int currentMilestone = 0;

    public CircularAgent(PApplet parent, Vec center, float radius, float speed, Vec color) {
        this.parent = parent;
        this.center = new Vec(center);
        this.radius = radius;
        this.speed = speed;
        this.color = new Vec(color);
    }

    public void spawn(Vec startPosition, List<Vec> newPath) {
        center.set(startPosition);
        path = new ArrayList<>(newPath);
        currentMilestone = 0;
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

    public void update(float dt) {
        if (isPaused) {
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // Reached next milestone?
            if (path.get(currentMilestone + 1).minus(center).norm() < MILESTONE_REACHED_SLACK) {
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

    public void draw() {
        // path
        parent.stroke(color.get(0), color.get(1), color.get(2));
        for (int i = 0; i < path.size() - 1; i++) {
            Vec v1 = path.get(i);
            Vec v2 = path.get(i + 1);
            parent.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
        }
        parent.noStroke();
        // agent
        parent.pushMatrix();
        parent.fill(color.get(0), color.get(1), color.get(2));
        parent.translate(0, center.get(1), center.get(0));
        parent.sphere(radius);
        parent.popMatrix();
        // next milestone
        if (currentMilestone < path.size() - 1) {
            Vec nextMilestonePosition = path.get(currentMilestone + 1);
            parent.pushMatrix();
            parent.fill(1, 0, 0);
            parent.translate(0, nextMilestonePosition.get(1), nextMilestonePosition.get(0));
            parent.sphere(radius);
            parent.popMatrix();
        }
    }
}
