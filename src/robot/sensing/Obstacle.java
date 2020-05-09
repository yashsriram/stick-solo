package robot.sensing;

import math.Vec;

public interface Obstacle {
    public boolean doesIntersect(final Vec p1, final Vec p2);
    public boolean doesIntersect(final Vec point, float margin);
    public void draw();
}
