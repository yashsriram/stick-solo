import org.ejml.simple.SimpleMatrix;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class NLinkAgent {
    private final PApplet applet;
    private final SimpleMatrix origin;
    private final List<Double> lengths = new ArrayList<>();
    private final List<Double> thetas = new ArrayList<>();

    public NLinkAgent(PApplet applet, double x, double y) {
        this.applet = applet;
        this.origin = new SimpleMatrix(new double[][]{{x}, {y}});
    }

    public void addLink(double theta, double length) {
        lengths.add(length);
        thetas.add(theta);
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
