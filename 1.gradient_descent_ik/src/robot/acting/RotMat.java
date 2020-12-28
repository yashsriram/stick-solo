package robot.acting;

import math.Mat;

public class RotMat {
    private static final Mat ROTATION_MATRIX = new Mat(new float[][]{
            {(float) Math.cos(0), (float) -Math.sin(0)},
            {(float) Math.sin(0), (float) Math.cos(0)}
    });

    public static Mat of(float angle) {
        float cosTheta = (float) Math.cos(angle);
        float sinTheta = (float) Math.sin(angle);
        ROTATION_MATRIX.set(0, 0, cosTheta);
        ROTATION_MATRIX.set(0, 1, -sinTheta);
        ROTATION_MATRIX.set(1, 0, sinTheta);
        ROTATION_MATRIX.set(1, 1, cosTheta);
        return ROTATION_MATRIX;
    }
}
