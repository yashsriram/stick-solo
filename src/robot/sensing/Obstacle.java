package robot.sensing;

import math.Vec;

public interface Obstacle {
    public boolean doesIntersect(final Vec p1, final Vec p2);

    public void draw();
}
