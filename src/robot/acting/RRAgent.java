package robot.acting;

import math.Vec;
import processing.core.PApplet;
import robot.planning.ik.RRIKSolver;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import static processing.core.PConstants.PI;

public class RRAgent {
    private final PApplet applet;

    private List<Vec> path;
    private int nextMilestone = 0;
    private final Vec goalJointTuple = new Vec(new float[]{Float.NaN, Float.NaN});

    private final Vec pivotPosition = new Vec(new float[]{0f, 0f});
    private final Vec jointTuple = new Vec(new float[]{0f, 0f});
    private final Vec lengths = new Vec(new float[]{0f, 0f});

    public RRAgent(PApplet applet, float l1, float l2, float q1InDegrees, float q2InDegrees, float x, float y, List<Vec> path) {
        this.applet = applet;
        this.lengths.headSet(l1, l2);
        this.pivotPosition.headSet(x, y);
        float q1 = q1InDegrees / 180f * PI;
        float q2 = q2InDegrees / 180f * PI;
        this.jointTuple.headSet(q1, q2);
        // First milestone
        this.path = new ArrayList<>(path);
        if (nextMilestone < path.size()) {
            goalJointTuple.headSet(RRIKSolver.solve_minusPItoPlusPI(pivotPosition, lengths, path.get(nextMilestone)));
        }
    }

    private float absMinClamp(float v, float min) {
        float absClamped = Math.max(Math.abs(v), min);
        float signClamped = Math.signum(v) * absClamped;
        return signClamped;
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

    private boolean isGoalJointVariablesValid() {
        return !Float.isNaN(goalJointTuple.get(0)) && !Float.isNaN(goalJointTuple.get(1));
    }

    public void update(float dt) {
        if (isGoalJointVariablesValid() && nextMilestone < path.size()) {
            // Reached next milestone
            if (Vec.dist(jointTuple, goalJointTuple) < 1e-2) {
                // Switch pivot
                List<Vec> ends = getLinkEnds();
                pivotPosition.headSet(ends.get(ends.size() - 1));
                // Reset joint angles
                float sumOfPreviousAngles = 0;
                for (int i = ends.size() - 1; i > 0; --i) {
                    Vec start = ends.get(i);
                    Vec end = ends.get(i - 1);
                    float angleWithX = (float) Math.atan2(end.get(1) - start.get(1), end.get(0) - start.get(0));
                    float angleWithPrevLink = angleWithX - sumOfPreviousAngles;
                    int jointVariable_iter = ends.size() - 1 - i;
                    jointTuple.set(jointVariable_iter, angleWithPrevLink);
                    sumOfPreviousAngles += angleWithX;
                }
                // Reverse lengths
                Vec lengthsCopy = new Vec(lengths);
                for (int i = lengthsCopy.getNumElements() - 1; i >= 0; i--) {
                    lengths.set(lengthsCopy.getNumElements() - 1 - i, lengthsCopy.get(i));
                }
                // If there is yet another milestone on the path
                if (nextMilestone + 1 < path.size()) {
                    // Update goal joint variables to take free end to that milestone
                    goalJointTuple.headSet(RRIKSolver.solve_minusPItoPlusPI(pivotPosition, lengths, path.get(nextMilestone + 1)));
                }
                nextMilestone++;
                return;
            }

            // Distance from goal is significant => Update all joint variables such that free end moves to goal
            for (int i = 0; i < jointTuple.getNumElements(); i++) {
                float vqi = absMinClamp(goalJointTuple.get(i) - jointTuple.get(i), 0.5f);
                jointTuple.set(i, jointTuple.get(i) + vqi * dt);
            }
        }
    }

    public void draw() {
        // path
        applet.stroke(1, 1, 0);
        for (int i = 0; i < path.size() - 1; i++) {
            Vec v1 = path.get(i);
            Vec v2 = path.get(i + 1);
            applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
        }
        applet.noStroke();
        applet.fill(1, 1, 0);
        for (Vec v : path) {
            applet.pushMatrix();
            applet.translate(0, v.get(1), v.get(0));
            applet.box(2);
            applet.popMatrix();
        }
        applet.noStroke();

        // Pivot
        applet.pushMatrix();
        applet.noStroke();
        applet.fill(0, 0, 1);
        applet.translate(0, pivotPosition.get(1), pivotPosition.get(0));
        applet.box(2);
        applet.popMatrix();

        // Links
        Vec start = new Vec(pivotPosition);
        Vec direction = new Vec(new float[]{1f, 0f});
        applet.noFill();
        if (isGoalJointVariablesValid()) {
            applet.stroke(1);
        } else {
            applet.stroke(1, 0, 0);
        }
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            // Rotate
            float theta = jointTuple.get(i);
            direction = RotMat.of(theta).mult(direction);
            // Translate
            float length = lengths.get(i);
            Vec end = start.plus(direction.scale(length));
            // Draw link
            applet.line(0, start.get(1), start.get(0), 0, end.get(1), end.get(0));
            start = end;
        }
    }
}
