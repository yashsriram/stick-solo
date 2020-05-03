package robot.planning.ik;

import math.Angle;
import math.Mat;
import math.Vec;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class RRIKSolver {
    public static Vec solve_minusPI_plusPI(final Vec pivot, final Vec lengths, final Vec goalPosition) {
        // RR manipulator specific inverse kinematic math
        Vec relativeGoalPosition = goalPosition.minus(pivot);
        float l = relativeGoalPosition.norm();
        if (l < 1e-6 && (lengths.get(0) == lengths.get(1))) {
            return new Vec(0, PApplet.PI);
        }
        float num = l * l + lengths.get(0) * lengths.get(0) - lengths.get(1) * lengths.get(1);
        float den = 2 * relativeGoalPosition.norm() * lengths.get(0);
        float q1 = (float) (Math.atan2(relativeGoalPosition.get(1), relativeGoalPosition.get(0)) - Math.acos(num / den));
        float q2 = (float) (Math.atan2(relativeGoalPosition.get(1) - lengths.get(0) * Math.sin(q1), relativeGoalPosition.get(0) - lengths.get(0) * Math.cos(q1)) - q1);

        // Make q1 b/w -PI, PI
        q1 = Angle.clamp_minusPI_plusPI(q1);
        // Make q2 b/w -PI, PI
        q2 = Angle.clamp_minusPI_plusPI(q2);

        return new Vec(q1, q2);
    }

    public static Vec jacobianTransposeStep(final Vec pivot, final Vec lengths, final Vec jointTuple, final Vec goalPosition) {
        // Find link ends coordinates w.r.t pivot
        List<Vec> a_i_0 = new ArrayList<>(Collections.singletonList(new Vec(pivot)));
        Vec prevEnd = new Vec(pivot);
        float angleWithX = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            angleWithX += jointTuple.get(i);
            prevEnd.set(0, prevEnd.get(0) + (float) (lengths.get(i) * Math.cos(angleWithX)));
            prevEnd.set(1, prevEnd.get(1) + (float) (lengths.get(i) * Math.sin(angleWithX)));
            a_i_0.add(new Vec(prevEnd));
        }
        // Free end coordinates
        Vec a_e_0 = a_i_0.get(a_i_0.size() - 1);
        // Building jacobian
        float[][] jacobianValues = new float[2][jointTuple.getNumElements()];
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            Vec a_ie_0 = a_e_0.minus(a_i_0.get(i));
            jacobianValues[0][i] = -a_ie_0.get(1);
            jacobianValues[1][i] = a_ie_0.get(0);
        }
        Mat jacobian = new Mat(jacobianValues);
        Vec delta_x = goalPosition.minus(a_e_0);
        return jacobian.transpose().mult(delta_x);
    }

    public static Vec pseudoInverseStep(final Vec pivot, final Vec lengths, final Vec jointTuple, final Vec goalPosition) {
        // Find link ends coordinates w.r.t pivot
        List<Vec> a_i_0 = new ArrayList<>(Collections.singletonList(new Vec(pivot)));
        Vec prevEnd = new Vec(pivot);
        float angleWithX = 0;
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            angleWithX += jointTuple.get(i);
            prevEnd.set(0, prevEnd.get(0) + (float) (lengths.get(i) * Math.cos(angleWithX)));
            prevEnd.set(1, prevEnd.get(1) + (float) (lengths.get(i) * Math.sin(angleWithX)));
            a_i_0.add(new Vec(prevEnd));
        }
        // Free end coordinates
        Vec a_e_0 = a_i_0.get(a_i_0.size() - 1);
        // Building jacobian
        float[][] jacobianValues = new float[2][jointTuple.getNumElements()];
        for (int i = 0; i < jointTuple.getNumElements(); i++) {
            Vec a_ie_0 = a_e_0.minus(a_i_0.get(i));
            jacobianValues[0][i] = -a_ie_0.get(1);
            jacobianValues[1][i] = a_ie_0.get(0);
        }
        Mat J = new Mat(jacobianValues);
        Mat J_T = J.transpose();
        Mat pseudoInverse = J_T.mult(J.mult(J_T).inverse());
        Vec delta_x = goalPosition.minus(a_e_0);
        Vec delta_jointTuple = pseudoInverse.mult(delta_x);
        return delta_jointTuple;
    }
}
