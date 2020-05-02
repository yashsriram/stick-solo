package robot.acting;

import math.Vec;
import processing.core.PApplet;
import robot.planning.ik.RRIKSolver;

import static processing.core.PConstants.PI;

public class RRAgent {
    private final PApplet applet;

    private final Vec holdingPosition = new Vec(new float[]{0f, 0f});
    private final Vec lengths = new Vec(new float[]{0f, 0f});
    private final Vec jointVariables = new Vec(new float[]{0f, 0f});
    private final Vec goalJointVariables = new Vec(new float[]{0f, 0f});

    public RRAgent(PApplet applet, float l1, float l2, float q1InDegrees, float q2InDegrees, float x, float y) {
        this.applet = applet;
        this.lengths.headSet(l1, l2);
        float q1 = q1InDegrees / 180f * PI;
        float q2 = q2InDegrees / 180f * PI;
        this.jointVariables.headSet(q1, q2);
        this.goalJointVariables.headSet(q1, q2);
        this.holdingPosition.headSet(x, y);
    }

    public void setGoalPosition(Vec goalPosition) {
        Vec newGoalJointVariables = RRIKSolver.solve_minusPItoPlusPI(holdingPosition, lengths, goalPosition);
        goalJointVariables.headSet(newGoalJointVariables.get(0), newGoalJointVariables.get(1));
    }

    private float absMinClamp(float v, float min) {
        float absClamped = Math.max(Math.abs(v), min);
        float signClamped = Math.signum(v) * absClamped;
        return signClamped;
    }

    public void update(float dt) {
        // If distance is not too small
        if (Vec.dist(jointVariables, goalJointVariables) > 1e-6) {
            // Update all joint variables
            for (int i = 0; i < jointVariables.getNumElements(); i++) {
                float vqi = absMinClamp(goalJointVariables.get(i) - jointVariables.get(i), 0.5f);
                jointVariables.set(i, jointVariables.get(i) + vqi * dt);
            }
        }
    }

    public void draw() {
        Vec start = new Vec(holdingPosition);
        Vec direction = new Vec(new float[]{1f, 0f});
        applet.stroke(1);
        for (int i = 0; i < jointVariables.getNumElements(); i++) {
            // Rotate
            float theta = jointVariables.get(i);
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
