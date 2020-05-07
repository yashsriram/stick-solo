package robot.sensing;

import math.Mat;
import math.Vec;
import processing.core.PApplet;

public class LineSegmentObstacle implements Obstacle {
    private final PApplet applet;
    public final Vec e1 = new Vec(0, 0);
    public final Vec e2 = new Vec(0, 0);
    private final Vec color = new Vec(1, 0, 1);

    public LineSegmentObstacle(PApplet applet, Vec e1, Vec e2, Vec color) {
        this.applet = applet;
        this.e1.headSet(e1);
        this.e2.headSet(e2);
        this.color.headSet(color);
    }

    private float findScale(final Vec original, final Vec scaled) {
        float scale = 0;
        int numNonZeroElements = 0;
        if (Math.abs(original.get(0)) > 1e-4) {
            scale += scaled.get(0) / original.get(0);
            numNonZeroElements++;
        }
        if (Math.abs(original.get(1)) > 1e-4) {
            scale += scaled.get(1) / original.get(1);
            numNonZeroElements++;
        }
        scale = scale / numNonZeroElements;
        return scale;
    }

    public boolean isInside(Vec p) {
        return false;
    }

    public boolean doesIntersect(final Vec p1, final Vec p2) {
        // A
        Mat A = new Mat(new float[][]{
                {p2.get(1) - p1.get(1), -(p2.get(0) - p1.get(0))},
                {e2.get(1) - e1.get(1), -(e2.get(0) - e1.get(0))}
        });
        // b
        Vec b = new Vec(p1.get(0) * p2.get(1) - p1.get(1) * p2.get(0), e1.get(0) * e2.get(1) - e1.get(1) * e2.get(0));
        if (Math.abs(A.determinant()) < 1e-6) {
            Vec v1 = p2.minus(e1).normalizeInPlace();
            Vec v2 = e2.minus(e1).normalizeInPlace();
            if (1 - Math.abs(v1.dot(v2)) < 1e-6) {
                // Coincident check if either p1 or p2 falls in b/w e1 and e2
                return Math.abs(e1.minus(p1).norm() + e2.minus(p1).norm() - e2.minus(e1).norm()) < 1e-6
                        || Math.abs(e1.minus(p2).norm() + e2.minus(p2).norm() - e2.minus(e1).norm()) < 1e-6;
            } else {
                // Parallel
                return false;
            }
        }
        // Ax = b
        Vec x = A.inverse().mult(b);

        // Is intersection b/w link end points?
        float endLength = (p2.minus(p1)).norm();
        Vec endUnitVec = (p2.minus(p1)).normalizeInPlace();
        Vec scaledEndUnitVec = x.minus(p1);
        float t1 = findScale(endUnitVec, scaledEndUnitVec);
        if (t1 < 0 || t1 > endLength) {
            return false;
        }

        // Is intersection b/w obs end points?
        float obsLength = (e2.minus(e1)).norm();
        Vec obsUnitVec = (e2.minus(e1)).normalizeInPlace();
        Vec scaledObsUnitVec = x.minus(e1);
        float t2 = findScale(obsUnitVec, scaledObsUnitVec);
        if (t2 < 0 || t2 > obsLength) {
            return false;
        }

        // All conditions are okay then return true
        return true;
    }

    public void draw() {
        applet.fill(color.get(0), color.get(1), color.get(2));
        applet.stroke(color.get(0), color.get(1), color.get(2));
        applet.line(0, e1.get(1), e1.get(0), 0, e2.get(1), e2.get(0));
        applet.pushMatrix();
        applet.translate(0, e1.get(1), e1.get(0));
        applet.box(0.5f);
        applet.popMatrix();
        applet.pushMatrix();
        applet.translate(0, e2.get(1), e2.get(0));
        applet.box(0.5f);
        applet.popMatrix();
    }
}
