import org.ejml.simple.SimpleMatrix;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class NLinkAgent {
    private final PApplet applet;
    private final SimpleMatrix origin;
    private final List<Double> lengths = new ArrayList<>();
    private final List<Double> thetas = new ArrayList<>();
    private final SimpleMatrix finish;

    public NLinkAgent(PApplet applet, double x, double y) {
        this.applet = applet;
        this.origin = new SimpleMatrix(new double[][]{{x}, {y}});
        this.finish = new SimpleMatrix(new double[][]{{20}, {0}});
    }

    public void addLink(double theta, double length) {
        lengths.add(length);
        thetas.add(theta);
    }

    public void update(float dt) {
        double num = finish.normF() * finish.normF() + lengths.get(0) * lengths.get(0) - lengths.get(1) * lengths.get(1);
        double den = 2 * finish.normF() * lengths.get(0);
        double goalTheta1 = Math.atan2(finish.get(1), finish.get(0)) - Math.acos(num / den);
        double goalTheta2 = Math.atan2(finish.get(1) - lengths.get(0) * Math.sin(goalTheta1), finish.get(0) - lengths.get(0) * Math.cos(goalTheta1))
                - goalTheta1;
        PApplet.println(thetas.get(0));
        PApplet.println(thetas.get(1));
        thetas.set(0, thetas.get(0) + 2 * dt * (goalTheta1 - thetas.get(0)));
        thetas.set(1, thetas.get(1) + 2 * dt * (goalTheta2 - thetas.get(1)));
    }

    private SimpleMatrix rotationMatrix(double theta) {
        return new SimpleMatrix(new double[][]
                {
                        {Math.cos(theta), -Math.sin(theta)},
                        {Math.sin(theta), Math.cos(theta)}
                }
        );
    }

    public void draw() {
        SimpleMatrix start = new SimpleMatrix(origin);
        SimpleMatrix direction = new SimpleMatrix(new float[][]{{1f}, {0f}});
        applet.stroke(1);
        for (int i = 0; i < thetas.size(); i++) {
            double theta = thetas.get(i);
            double length = lengths.get(i);
            direction = rotationMatrix(theta).mult(direction);
            SimpleMatrix end = start.plus(direction.scale(length));
            applet.line(
                    0, (float) start.get(1), (float) start.get(0),
                    0, (float) end.get(1), (float) end.get(0)
            );
            start = end;
        }
    }
}
