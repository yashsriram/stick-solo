package robot.acting;

import math.Angle;
import math.Vec;
import processing.core.PApplet;
import processing.core.PShape;
import robot.planning.ik.NRIKSolver;
import robot.sensing.PositionConfigurationSpace;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class NRIterativeBodyPartAgent {
    public enum IKMethod {JACOBIAN_TRANSPOSE, PSEUDO_INVERSE}

    public final int id;

    public static boolean DRAW_LIMBS = true;
    public static boolean DRAW_GOAL = false;
    public static float MILESTONE_REACHED_SLACK = 1f;
    public static float JERK_THRESHOLD = 1e-6f;
    public static IKMethod METHOD = IKMethod.JACOBIAN_TRANSPOSE;

    private final PApplet applet;
    private final int N;

    private boolean isStraight = true;
    private final Vec pivot = new Vec(0f, 0f);
    private final Vec lengths;
    private final Vec jointTuple;
    private final Vec goal = new Vec(0, 0);

    private PShape shape1;
    private PShape shape2;

    public NRIterativeBodyPartAgent(PApplet applet, int id, int N, PShape shape1, PShape shape2) {
        this.applet = applet;
        this.id = id;
        this.N = N;
        this.lengths = new Vec(new float[N]);
        this.jointTuple = new Vec(new float[N]);
        this.shape1 = shape1;
        this.shape2 = shape2;
    }

    public NRIterativeBodyPartAgent(PApplet applet, int id, int N) {
        this.applet = applet;
        this.id = id;
        this.N = N;
        this.lengths = new Vec(new float[N]);
        this.jointTuple = new Vec(new float[N]);
    }

    public float totalArmLength() {
        float len = 0;
        for (int i = 0; i < lengths.getNumElements(); i++) {
            len += lengths.get(i);
        }
        return len;
    }

    private Vec getFreeEnd() {
        Vec freeEnd = new Vec(pivot);
        float angleWithX = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            angleWithX += jointTuple.get(i);
            freeEnd.set(0, freeEnd.get(0) + (float) (lengths.get(i) * Math.cos(angleWithX)));
            freeEnd.set(1, freeEnd.get(1) + (float) (lengths.get(i) * Math.sin(angleWithX)));
        }
        return freeEnd;
    }

    public void spawn(Vec pivot, Vec lengths, Vec jointTuple) {
        if (lengths.getNumElements() != N || jointTuple.getNumElements() != N) {
            PApplet.println("Invalid spawn parameters");
            return;
        }
        this.pivot.headSet(pivot.get(0), pivot.get(1));
        this.lengths.headSet(lengths);
        this.jointTuple.headSet(jointTuple);
        this.goal.headSet(getFreeEnd());
        this.isStraight = true;
    }

    public void setGoal(Vec goal) {
        this.goal.headSet(goal);
    }

    public Vec getGoal() {
        return goal;
    }

    private List<Vec> getLinkEnds() {
        List<Vec> ends = new ArrayList<>(Collections.singletonList(new Vec(pivot)));
        Vec prevEnd = new Vec(pivot);
        float sumOfPreviousAngles = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            sumOfPreviousAngles += jointTuple.get(i);
            prevEnd.set(0, prevEnd.get(0) + (float) (lengths.get(i) * Math.cos(sumOfPreviousAngles)));
            prevEnd.set(1, prevEnd.get(1) + (float) (lengths.get(i) * Math.sin(sumOfPreviousAngles)));
            ends.add(new Vec(prevEnd));
        }
        return ends;
    }

    public boolean doesIntersect(PositionConfigurationSpace cs) {
        List<Vec> points = this.getLinkEnds();
        boolean intersect = false;
        for (int i = 0; i < points.size() - 1; i++) {
            intersect = intersect || cs.doesIntersect(points.get(i), 5);
            if (intersect) {
                return true;
            }
        }
        for (int i = 0; i < points.size() - 1; i++) {
            intersect = intersect || cs.doesIntersect(points.get(i), points.get(i + 1));
            if (intersect) {
                return true;
            }
        }
        return intersect;
    }

    public void switchPivot() {
        List<Vec> ends = getLinkEnds();
        // Switch pivot
        pivot.headSet(ends.get(ends.size() - 1));
        // Reset joint angles
        float prevLinkAngleWithX = 0;
        for (int i = ends.size() - 1; i > 0; --i) {
            Vec start = ends.get(i);
            Vec end = ends.get(i - 1);
            float angleWithX = (float) Math.atan2(end.get(1) - start.get(1), end.get(0) - start.get(0));
            float angleWithPrevLink = Angle.clamp_minusPI_plusPI(angleWithX - prevLinkAngleWithX);
            int jointVariable_iter = ends.size() - 1 - i;
            jointTuple.set(jointVariable_iter, angleWithPrevLink);
            prevLinkAngleWithX = angleWithX;
        }
        // Reverse lengths
        Vec lengthsCopy = new Vec(lengths);
        for (int i = lengthsCopy.getNumElements() - 1; i >= 0; i--) {
            lengths.set(lengthsCopy.getNumElements() - 1 - i, lengthsCopy.get(i));
        }
        isStraight = !isStraight;
    }

    public boolean isStraight() {
        return isStraight;
    }

    public Vec getJointTuple() {
        return jointTuple;
    }

    public boolean update(float dt, float minSpeedLimit) {
        // Reached next milestone
        if (Vec.dist(getFreeEnd(), goal) < MILESTONE_REACHED_SLACK) {
            return true;
        }
        // Distance from next milestone is significant => Update all joint variables such that free end moves to next milestone
        Vec deltaJointTupleUnscaled;
        if (METHOD == IKMethod.PSEUDO_INVERSE) {
            deltaJointTupleUnscaled = NRIKSolver.pseudoInverseStep(getLinkEnds(), jointTuple, goal);
        } else {
            deltaJointTupleUnscaled = NRIKSolver.jacobianTransposeStep(getLinkEnds(), jointTuple, goal);
        }
        // Scale the delta
        Vec deltaJointTuple = deltaJointTupleUnscaled.scaleInPlace(dt);
        // If stuck in a singular configuration the give a little jerk
        if (deltaJointTuple.norm() < JERK_THRESHOLD) {
            for (int i = 0; i < jointTuple.getNumElements(); i++) {
                deltaJointTuple.set(i, deltaJointTuple.get(i) + applet.random(1f / (jointTuple.getNumElements() + 1)));
            }
        }
        // If too speed is too low, increase it to a minimum
        if (deltaJointTuple.norm() < minSpeedLimit) {
            deltaJointTuple.normalizeInPlace().scaleInPlace(minSpeedLimit);
        }
        jointTuple.plusInPlace(deltaJointTuple);
        return false;
    }

    public void draw(Vec color) {
        if (DRAW_GOAL) {
            // Goal milestone
            applet.noStroke();
            applet.pushMatrix();
            applet.fill(1, 0, 0);
            applet.translate(0, goal.get(1), goal.get(0));
            applet.box(1);
            applet.popMatrix();
        }

        if (DRAW_LIMBS && this.shape1 != null && this.shape2 != null) {
            drawLimb(color);
            return;
        }

        // Pivot
        applet.pushMatrix();
        applet.noStroke();
        applet.fill(0, 1, 0);
        applet.translate(0, pivot.get(1), pivot.get(0));
        applet.box(1.5f);
        applet.popMatrix();

        // Links
        Vec start = new Vec(pivot);
        Vec direction = new Vec(1f, 0f);
        applet.noFill();
        applet.stroke(1);
        applet.strokeWeight(4);
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            // Rotate
            float theta = jointTuple.get(i);
            direction = RotMat.of(theta).mult(direction);
            // Translate
            float length = lengths.get(i);
            Vec end = start.plus(direction.scale(length));
            // Draw link
            applet.stroke(1);
            applet.line(0, start.get(1), start.get(0), 0, end.get(1), end.get(0));
            applet.noStroke();
            applet.pushMatrix();
            applet.fill(0, 0, 1);
            applet.translate(0, end.get(1), end.get(0));
            applet.box(1.5f);
            applet.popMatrix();
            start = end;
        }
        applet.strokeWeight(1);

    }

    public void draw() {
        if (DRAW_GOAL) {
            // Goal milestone
            applet.noStroke();
            applet.pushMatrix();
            applet.fill(1, 0, 0);
            applet.translate(0, goal.get(1), goal.get(0));
            applet.box(1);
            applet.popMatrix();
        }

        // Pivot
        applet.pushMatrix();
        applet.noStroke();
        applet.fill(0, 1, 0);
        applet.translate(0, pivot.get(1), pivot.get(0));
        applet.box(1.5f);
        applet.popMatrix();

        if (DRAW_LIMBS && this.shape1 != null && this.shape2 != null) {
            drawLimb();
            return;
        }

        // Links
        Vec start = new Vec(pivot);
        Vec direction = new Vec(1f, 0f);
        applet.noFill();
        applet.stroke(1);
        applet.strokeWeight(4);
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            // Rotate
            float theta = jointTuple.get(i);
            direction = RotMat.of(theta).mult(direction);
            // Translate
            float length = lengths.get(i);
            Vec end = start.plus(direction.scale(length));
            // Draw link
            applet.stroke(1);
            applet.line(0, start.get(1), start.get(0), 0, end.get(1), end.get(0));
            applet.noStroke();
            applet.pushMatrix();
            applet.fill(0, 0, 1);
            applet.translate(0, end.get(1), end.get(0));
            applet.box(1.5f);
            applet.popMatrix();
            start = end;
        }
        applet.strokeWeight(1);
    }

    private void drawLimb(Vec color) {
        List<Vec> ends = getLinkEnds();
        boolean changeShape = this.isStraight;
        this.shape1.setFill(applet.color(color.get(0), color.get(1), color.get(2)));
        this.shape2.setFill(applet.color(color.get(0), color.get(1), color.get(2)));
        for (int i = 0; i < ends.size() - 1; i++) {
            Vec start;
            Vec end;
            if (this.isStraight) {
                start = ends.get(i);
                end = ends.get(i + 1);
            } else {
                start = ends.get(i + 1);
                end = ends.get(i);
            }
            float theta = PApplet.atan2(end.get(1) - start.get(1), end.get(0) - start.get(0));
            PShape shape;
            if (changeShape) {
                shape = shape1;
            } else {
                shape = shape2;
            }
            applet.pushMatrix();
//            applet.translate( 0, (start.get(1)+end.get(1))/2, (start.get(0)+end.get(0))/2);
            applet.translate(0, end.get(1), end.get(0));
//            applet.rotateX(PApplet.PI);
//            applet.rotateY(PApplet.PI/2);
            applet.rotateX(-theta);
            applet.shape(shape);
            applet.popMatrix();
            changeShape = !changeShape;
            start = end;
        }
    }

    private void drawLimb() {
        List<Vec> ends = getLinkEnds();
        boolean changeShape = this.isStraight;
        for (int i = 0; i < ends.size() - 1; i++) {
            Vec start;
            Vec end;
            if (this.isStraight) {
                start = ends.get(i);
                end = ends.get(i + 1);
            } else {
                start = ends.get(i + 1);
                end = ends.get(i);
            }
            float theta = PApplet.atan2(end.get(1) - start.get(1), end.get(0) - start.get(0));
            PShape shape;
            if (changeShape) {
                shape = shape1;
            } else {
                shape = shape2;
            }
            applet.pushMatrix();
//            applet.translate( 0, (start.get(1)+end.get(1))/2, (start.get(0)+end.get(0))/2);
            applet.translate(0, end.get(1), end.get(0));
//            applet.rotateX(PApplet.PI);
//            applet.rotateY(PApplet.PI/2);
            applet.rotateX(-theta);
            applet.shape(shape);
            applet.popMatrix();
            changeShape = !changeShape;
            start = end;
        }
    }

}
