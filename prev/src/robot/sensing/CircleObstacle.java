package robot.sensing;

import math.Vec;
import processing.Processing;
import processing.core.PApplet;

public class CircleObstacle implements Obstacle {
    private final PApplet applet;
    public final Vec center = new Vec(0, 0);
    public final float radius;
    private final Vec color = new Vec(1, 0, 1);

    public CircleObstacle(PApplet applet, Vec center, float radius, Vec color) {
        this.applet = applet;
        this.center.headSet(center);
        this.radius = radius;
        this.color.headSet(color);
    }

    public boolean doesIntersect(final Vec p1, final Vec p2) {
        Vec pb_pa = p2.minus(p1);
        Vec pa_pc = p1.minus(center);
        float r = radius;
        float a = pb_pa.dot(pb_pa);
        float c = pa_pc.dot(pa_pc) - r * r;
        float b = 2 * pb_pa.dot(pa_pc);
        float discriminant = b * b - 4 * a * c;
        if (discriminant >= 0) {
            float t1 = (float) ((-b + Math.sqrt(discriminant)) / (2 * a));
            float t2 = (float) ((-b - Math.sqrt(discriminant)) / (2 * a));
            // Intersection with line segment only possible iff at least one of the solutions lies in [0, 1]
            return (0 <= t1 && t1 <= 1) || (0 <= t2 && t2 <= 1);
        }
        // Discriminant < 0 => no intersection
        return false;
    }

    public boolean doesIntersect(final Vec point, float margin) {
        if (Vec.dist(this.center, point) <= (this.radius + margin)) {
            return true;
        }
        return false;
    }

    public void draw() {
        applet.noFill();
        Processing.circleYZ(applet, center.get(0), center.get(1), radius, 40);
    }
}
