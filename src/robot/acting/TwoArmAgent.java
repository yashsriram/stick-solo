package robot.acting;

import math.Vec;
import processing.core.PApplet;

import java.util.List;

public class TwoArmAgent {
    private final PApplet applet;
    private final NRIterativeAgent arm1;
    private final NRIterativeAgent arm2;
    private final Vec neck = new Vec(0, 0);
    Vec newMilestone = new Vec(8, -8);

    public TwoArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeAgent(applet, 2);
        this.arm2 = new NRIterativeAgent(applet, 2);

        arm1.spawn(List.of(neck), new Vec(10, 10), new Vec(-PApplet.PI * 0.75f, PApplet.PI * 0.25f));
        arm2.spawn(List.of(neck), new Vec(10, 10), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
    }

    public void move1() {
        arm1.addMilestone(newMilestone, true);
    }

    public void move2() {
        arm1.addMilestone(newMilestone.minus(neck).scaleInPlace(0.5f), true);
        arm2.addMilestone(newMilestone.minus(neck).scaleInPlace(0.5f), true);
    }

    public void move3() {
    }

    public boolean update(float dt) {
        return arm1.update(dt) || arm2.update(dt);
    }

    public void draw() {
        arm1.draw();
        arm2.draw();
    }
}
