package robot.sensing;

import math.Vec;
import processing.core.PApplet;

import java.util.List;

public class PositionConfigurationSpace {
    private final PApplet applet;
    private final List<Obstacle> obstacles;

    public PositionConfigurationSpace(PApplet applet, List<Obstacle> obstacles) {
        this.applet = applet;
        this.obstacles = obstacles;
    }

    public boolean doesIntersect(final Vec p1, final Vec p2) {
        for (Obstacle obs : obstacles) {
            if (obs.doesIntersect(p1, p2)) {
                return true;
            }
        }
        return false;
    }

    public boolean isInside(final Vec p) {
        for (Obstacle obs : obstacles) {
            if (obs.isInside(p)) {
                return true;
            }
        }
        return false;
    }

    public void draw() {
        for (Obstacle obs : obstacles) {
            obs.draw();
        }
    }
}
