package robot.acting;

import math.Vec;
import processing.core.PApplet;

import java.util.List;

public class TwoArmAgent {
    private enum State {LEAD_ARM_SET, LEAD_ARM_GO, NECK_SET, NECK_GO, FOLLOW_ARM_SET, FOLLOW_ARM_GO}

    private final PApplet applet;
    private final NRIterativeBodyPartAgent arm1;
    private final NRIterativeBodyPartAgent arm2;
    private final Vec neck = new Vec(0, 0);

    private State state = State.LEAD_ARM_SET;
    Vec newMilestone = new Vec(20, -20);
    Vec neckMilestone = new Vec(neck);

    public TwoArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeBodyPartAgent(applet, 2);
        this.arm2 = new NRIterativeBodyPartAgent(applet, 2);

        arm1.spawn(List.of(neck), new Vec(20, 20), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
        arm2.spawn(List.of(neck), new Vec(20, 20), new Vec(-PApplet.PI * 0.75f, PApplet.PI * 0.25f));
        NRIterativeBodyPartAgent.DRAW_PATH = false;
    }

    public boolean update(float dt) {
        PApplet.println(state);
        switch (state) {
            case LEAD_ARM_SET:
                if (!arm1.isPivotTheOriginalOne()) {
                    arm1.switchPivot();
                }
                arm1.addMilestone(newMilestone);
                state = State.LEAD_ARM_GO;
                break;
            case LEAD_ARM_GO:
                arm1.update(dt);
//                if (Vec.dist(arm1.getInitialPivotCurrentPosition(), newMilestone) < NRIterativeAgent.MILESTONE_REACHED_SLACK) {
//                    state = State.NECK_SET;
//                }
                break;
            case NECK_SET:
                Vec neckToMilestone = newMilestone.minus(neck);
                float neckToHoldDist = neckToMilestone.norm();
                if (neckToHoldDist > 15) {
                    neckToMilestone.normalizeInPlace().scaleInPlace(neckToHoldDist - 15);
                    neckMilestone = neck.plus(neckToMilestone);
                }
                if (arm1.isPivotTheOriginalOne()) {
                    arm1.switchPivot();
                }
                if (arm2.isPivotTheOriginalOne()) {
                    arm2.switchPivot();
                }
                arm1.addMilestone(neckMilestone);
                arm2.addMilestone(neckMilestone);
                state = State.NECK_GO;
                break;
            case NECK_GO:
                arm1.update(dt);
                arm2.update(dt);
//                if (Vec.dist(arm1.getFreeEnd(), neckMilestone) < NRIterativeAgent.MILESTONE_REACHED_SLACK
//                        && Vec.dist(arm2.getFreeEnd(), neckMilestone) < NRIterativeAgent.MILESTONE_REACHED_SLACK) {
//                    state = State.FOLLOW_ARM_SET;
//                }
//                break;
//            case FOLLOW_ARM_SET:
//                if (!arm2.isOriginalPivot()) {
//                    arm2.switchPivot();
//                }
//                arm2.addMilestone(newMilestone);
//                state = State.FOLLOW_ARM_GO;
//                break;
//            case FOLLOW_ARM_GO:
//                arm2.update(dt);
//                if (Vec.dist(arm2.getFreeEnd(), newMilestone) < NRIterativeAgent.MILESTONE_REACHED_SLACK) {
//                    state = State.LEAD_ARM_SET;
//                }
//                break;
        }
        return false;
    }

    public void draw() {
        arm1.draw();
        arm2.draw();
    }
}
