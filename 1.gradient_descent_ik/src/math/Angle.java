package math;

public class Angle {
    public static float clamp_minusPI_plusPI(float angle) {
        // Clamp angle b/w -2PI, 2PI
        if (angle > 2 * Math.PI) {
            angle = (float) (angle % (2 * Math.PI));
        } else if (angle < -2 * Math.PI) {
            angle = (float) (angle % (2 * Math.PI));
        }
        // Clamp angle b/w -PI, PI
        if (angle > Math.PI) {
            angle -= 2 * Math.PI;
        } else if (angle <= -Math.PI) {
            angle += 2 * Math.PI;
        }
        return angle;
    }
}
