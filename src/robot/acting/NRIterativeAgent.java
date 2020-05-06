package robot.acting;

import math.Angle;
import math.Vec;
import processing.core.PApplet;
import robot.planning.ik.RRIKSolver;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class NRIterativeAgent {
    public static float MILESTONE_REACHED_SLACK = 1f;
    public static float JERK_THRESHOLD = 1e-6f;

    private final PApplet applet;
    public boolean isPaused = false;

    private final int N;
    private final Vec pivotPosition = new Vec(0f, 0f);
    private final Vec lengths;
    private final Vec jointTuple;

    private List<Vec> path = new ArrayList<>();
    private int nextMilestone = 1;

    public float minSpeedLimit = 1e-2f;
    public boolean isMinSpeedLimitCalculated = false;

    public NRIterativeAgent(PApplet applet, int N) {
        this.applet = applet;
        this.N = N;
        this.lengths = new Vec(new float[N]);
        this.jointTuple = new Vec(new float[N]);
    }

    public void spawn(List<Vec> path, Vec lengths, Vec jointTuple) {
        if (lengths.getNumElements() != N || jointTuple.getNumElements() != N) {
            PApplet.println("Invalid spawn parameters");
            return;
        }
        if (path.size() > 0) {
            Vec firstMilestone = path.get(0);
            this.pivotPosition.headSet(firstMilestone.get(0), firstMilestone.get(1));
        } else {
            this.pivotPosition.headSet(0, 0);
        }
        this.lengths.headSet(lengths);
        this.jointTuple.headSet(jointTuple);
        this.path = new ArrayList<>(path);
        this.nextMilestone = 1;
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

    public void update(float dt) {
        if (isPaused) {
            return;
        }
        if (nextMilestone < path.size()) {
            // Reached next milestone
            if (Vec.dist(getFreeEnd(), path.get(nextMilestone)) < MILESTONE_REACHED_SLACK) {
                // Switch pivot
                List<Vec> ends = getLinkEnds();
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
                nextMilestone++;
                isMinSpeedLimitCalculated = false;
                return;
            }
            // Distance from next milestone is significant => Update all joint variables such that free end moves to next milestone
            Vec deltaJointTupleUnscaled = RRIKSolver.jacobianTransposeStep(pivotPosition, lengths, jointTuple, path.get(nextMilestone));
            Vec deltaJointTuple = deltaJointTupleUnscaled.scaleInPlace(dt);
            if (!isMinSpeedLimitCalculated) {
                minSpeedLimit = deltaJointTuple.norm();
                isMinSpeedLimitCalculated = true;
            }
            // If too speed is too low, increase it to a minimum
            if (deltaJointTuple.norm() < minSpeedLimit) {
                deltaJointTuple.normalizeInPlace().scaleInPlace(minSpeedLimit);
            }
            // If stuck in a singular configuration the give a little jerk
            if (deltaJointTuple.norm() < JERK_THRESHOLD) {
                deltaJointTuple.plusInPlace(new Vec(applet.random(0.5f), applet.random(0.5f)));
            }
            jointTuple.plusInPlace(deltaJointTuple);
        }
    }

    public void draw() {
        // path
        applet.stroke(1);
        for (int i = 0; i < path.size() - 1; i++) {
            Vec v1 = path.get(i);
            Vec v2 = path.get(i + 1);
            applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
        }
        applet.noStroke();
        applet.fill(1);
        for (Vec v : path) {
            applet.pushMatrix();
            applet.translate(0, v.get(1), v.get(0));
            applet.box(1);
            applet.popMatrix();
        }
        applet.noStroke();

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
