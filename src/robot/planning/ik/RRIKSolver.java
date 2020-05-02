package robot.planning.ik;

import math.Vec;
import processing.core.PApplet;

public class RRIKSolver {
    public static Vec solve_minusPItoPlusPI(final Vec pivot, final Vec lengths, final Vec goalPosition) {
        // RR IK specific math
        Vec relativeGoalPosition = goalPosition.minus(pivot);
        float l = relativeGoalPosition.norm();
        if (l < 1e-6 && (lengths.get(0) == lengths.get(1))) {
            return new Vec(new float[]{0, PApplet.PI});
        }
        float num = l * l + lengths.get(0) * lengths.get(0) - lengths.get(1) * lengths.get(1);
        float den = 2 * relativeGoalPosition.norm() * lengths.get(0);
        float q1 = (float) (Math.atan2(relativeGoalPosition.get(1), relativeGoalPosition.get(0)) - Math.acos(num / den));
        float q2 = (float) (Math.atan2(relativeGoalPosition.get(1) - lengths.get(0) * Math.sin(q1), relativeGoalPosition.get(0) - lengths.get(0) * Math.cos(q1)) - q1);

        return new Vec(new float[]{q1, q2});
    }
}
