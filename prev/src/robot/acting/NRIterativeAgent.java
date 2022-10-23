package robot.acting;

import math.Angle;
import math.Vec;
import processing.core.PApplet;
import robot.planning.ik.NRIKSolver;
import robot.planning.prm.Milestone;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class NRIterativeAgent {
    public enum IKMethod {JACOBIAN_TRANSPOSE, PSEUDO_INVERSE}

    public static float MILESTONE_REACHED_SLACK = 1f;
    public static float JERK_THRESHOLD = 1e-6f;
    public static IKMethod METHOD = IKMethod.JACOBIAN_TRANSPOSE;

    public boolean isPaused = false;

    private final PApplet applet;
    private final int N;

    private final Vec pivotPosition = new Vec(0f, 0f);
    private final Vec lengths;
    private final Vec jointTuple;
    private final List<Vec> freeEndPath = new ArrayList<>();

    private List<Milestone> path = new ArrayList<>();
    private int nextMilestone = 1;

    private float minSpeedLimit = 0;
    private boolean isMinSpeedLimitCalculated = false;

    public NRIterativeAgent(PApplet applet, int N) {
        this.applet = applet;
        this.N = N;
        this.lengths = new Vec(new float[N]);
        this.jointTuple = new Vec(new float[N]);
    }

    public void spawn(List<Milestone> path, Vec lengths, Vec jointTuple) {
        if (lengths.getNumElements() != N || jointTuple.getNumElements() != N) {
            PApplet.println("Invalid spawn parameters");
            return;
        }
        if (path.size() > 0) {
            Vec firstMilestone = path.get(0).position;
            this.pivotPosition.headSet(firstMilestone.get(0), firstMilestone.get(1));
        } else {
            this.pivotPosition.headSet(0, 0);
        }
        this.lengths.headSet(lengths);
        this.jointTuple.headSet(jointTuple);
        this.path = new ArrayList<>(path);
        this.nextMilestone = 1;
        this.freeEndPath.clear();
    }

    private List<Vec> getLinkEnds() {
        List<Vec> ends = new ArrayList<>(Collections.singletonList(new Vec(pivotPosition)));
        Vec prevEnd = new Vec(pivotPosition);
        float sumOfPreviousAngles = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            sumOfPreviousAngles += jointTuple.get(i);
            prevEnd.set(0, prevEnd.get(0) + (float) (lengths.get(i) * Math.cos(sumOfPreviousAngles)));
            prevEnd.set(1, prevEnd.get(1) + (float) (lengths.get(i) * Math.sin(sumOfPreviousAngles)));
            ends.add(new Vec(prevEnd));
        }
        return ends;
    }

    private Vec getFreeEnd() {
        Vec freeEnd = new Vec(pivotPosition);
        float angleWithX = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            angleWithX += jointTuple.get(i);
            freeEnd.set(0, freeEnd.get(0) + (float) (lengths.get(i) * Math.cos(angleWithX)));
            freeEnd.set(1, freeEnd.get(1) + (float) (lengths.get(i) * Math.sin(angleWithX)));
        }
        return freeEnd;
    }

    private void switchPivot() {
        List<Vec> ends = getLinkEnds();
        // Switch pivot
        pivotPosition.headSet(ends.get(ends.size() - 1));
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
        freeEndPath.clear();
    }

    public boolean update(float dt) {
        if (isPaused) {
            return false;
        }
        if (nextMilestone < path.size()) {
            // Reached next milestone
            Vec freeEnd = getFreeEnd();
            freeEndPath.add(new Vec(freeEnd));
            if (Vec.dist(freeEnd, path.get(nextMilestone).position) < MILESTONE_REACHED_SLACK) {
                switchPivot();
                nextMilestone++;
                isMinSpeedLimitCalculated = false;
                return true;
            }
            // Distance from next milestone is significant => Update all joint variables such that free end moves to next milestone
            Vec deltaJointTupleUnscaled;
            if (METHOD == IKMethod.PSEUDO_INVERSE) {
                deltaJointTupleUnscaled = NRIKSolver.pseudoInverseStep(getLinkEnds(), jointTuple, path.get(nextMilestone).position);
            } else {
                deltaJointTupleUnscaled = NRIKSolver.jacobianTransposeStep(getLinkEnds(), jointTuple, path.get(nextMilestone).position);
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
            if (!isMinSpeedLimitCalculated) {
                minSpeedLimit = deltaJointTuple.norm();
                isMinSpeedLimitCalculated = true;
            }
            if (deltaJointTuple.norm() < minSpeedLimit) {
                deltaJointTuple.normalizeInPlace().scaleInPlace(minSpeedLimit);
            }
            jointTuple.plusInPlace(deltaJointTuple);
        }
        return false;
    }

    public void draw() {
        // Path
        applet.stroke(0.3f);
        for (int i = 0; i < path.size() - 1; i++) {
            Vec v1 = path.get(i).position;
            Vec v2 = path.get(i + 1).position;
            applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
        }
        applet.noStroke();
        applet.fill(0.3f);
        for (Milestone milestone : path) {
            Vec v = milestone.position;
            applet.pushMatrix();
            applet.translate(0, v.get(1), v.get(0));
            applet.box(1);
            applet.popMatrix();
        }
        applet.noStroke();

        // Free end path
        applet.stroke(1, 1, 0);
        for (int i = 0; i < freeEndPath.size() - 1; i++) {
            Vec v1 = freeEndPath.get(i);
            Vec v2 = freeEndPath.get(i + 1);
            applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
        }
        applet.noStroke();

        // Goal milestone
        if (nextMilestone < path.size()) {
            applet.fill(1, 0, 0);
            applet.pushMatrix();
            applet.translate(0, path.get(nextMilestone).position.get(1), path.get(nextMilestone).position.get(0));
            applet.box(1);
            applet.popMatrix();
        }

        // Pivot
        applet.pushMatrix();
        applet.noStroke();
        applet.fill(0, 1, 0);
        applet.translate(0, pivotPosition.get(1), pivotPosition.get(0));
        applet.box(1.5f);
        applet.popMatrix();

        // Links
        Vec start = new Vec(pivotPosition);
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
            applet.fill(0, 0, 1);
            applet.pushMatrix();
            applet.translate(0, end.get(1), end.get(0));
            applet.box(1.5f);
            applet.popMatrix();
            start = end;
        }
        applet.strokeWeight(1);
    }
}
