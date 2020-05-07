package robot.planning.ik;

import math.Angle;
import math.Mat;
import math.Vec;
import processing.core.PApplet;

import java.util.List;

public class NRIKSolver {
    public static Vec solve_RR_minusPI_plusPI(final Vec pivot, final Vec lengths, final Vec goalPosition) {
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

    public static Vec jacobianTransposeStep(final List<Vec> a_i_0, final Vec jointTuple, final Vec goalPosition) {
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

    public static Vec pseudoInverseStep(List<Vec> a_i_0, final Vec jointTuple, final Vec goalPosition) {
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
        Mat J_J_T = J.mult(J_T);
        if (J_J_T.determinant() < 1e-6) {
            Vec zero = new Vec(jointTuple);
            for (int i = 0; i < jointTuple.getNumElements(); i++) {
                zero.set(i, 0);
            }
            return zero;
        }
        Mat pseudoInverse = J_T.mult(J_J_T.inverse());
        Vec delta_x = goalPosition.minus(a_e_0);
        return pseudoInverse.mult(delta_x);
    }
}
