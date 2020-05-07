package robot.acting;

import math.Vec;
import processing.core.PApplet;

public class TwoArmAgent {
    public static float MIN_LIMB_SPEED = 0.003f;
    public static float NECK_SPEED = 0.005f;

    public boolean isPaused = false;

    private final PApplet applet;
    private final NRIterativeBodyPartAgent arm1;
    private final NRIterativeBodyPartAgent arm2;

    private int state = 0;
    private final Vec neck = new Vec(-40, 40);
    private final Vec goal = new Vec(-40, 40);
    private final Vec neckGoal = new Vec(neck);

    public TwoArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeBodyPartAgent(applet, 2);
        this.arm2 = new NRIterativeBodyPartAgent(applet, 2);

        arm1.spawn(neck, new Vec(20, 20), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
        arm2.spawn(neck, new Vec(20, 20), new Vec(-PApplet.PI * 0.75f, PApplet.PI * 0.25f));
    }

    public boolean update(float dt) {
        if (isPaused) {
            return false;
        }
        PApplet.println(state);
        switch (state) {
            case 0:
                goal.headSet(goal.get(0) + 15, goal.get(1) - 15);
                if (!arm1.isStraight()) {
                    arm1.switchPivot();
                }
                arm1.setGoal(goal);
                state++;
                break;
            case 1:
                if (arm1.update(dt, MIN_LIMB_SPEED)) {
                    state++;
                }
                break;
            case 2:
                Vec neckToGoal = goal.minus(neck);
                float neckToGoalDist = neckToGoal.norm();
                if (neckToGoalDist > 15) {
                    neckToGoal.normalizeInPlace().scaleInPlace(neckToGoalDist - 15);
                    neckGoal.headSet(neck.plus(neckToGoal));;
                }
                if (arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (arm2.isStraight()) {
                    arm2.switchPivot();
                }
                state++;
                break;
            case 3:
                neck.plusInPlace(neckGoal.minus(neck).scaleInPlace(NECK_SPEED));
                arm1.setGoal(neck);
                arm2.setGoal(neck);
                while (true) {
                    boolean arm1Ok = arm1.update(dt, MIN_LIMB_SPEED);
                    boolean arm2Ok = arm2.update(dt, MIN_LIMB_SPEED);
                    if (arm1Ok && arm2Ok) {
                        break;
                    }
                }
                if (Vec.dist(neck, neckGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK) {
                    state++;
                }
                break;
            case 4:
                if (!arm2.isStraight()) {
                    arm2.switchPivot();
                }
                arm2.setGoal(goal);
                state++;
                break;
            case 5:
                if (arm2.update(dt, MIN_LIMB_SPEED)) {
                    state = 0;
                }
                break;
        }
        return false;
    }

    public void draw() {
        arm1.draw();
        arm2.draw();
    }
}
