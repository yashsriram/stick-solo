package robot.acting;

import math.Vec;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class TwoArmAgent {
    public static boolean DRAW_PATH = true;
    public static float MIN_LIMB_SPEED = 0.006f;
    public static float NECK_SPEED = 0.01f;
    public static float NECK_SYNC_ITERATIONS = 100;

    public boolean isPaused = false;

    private final PApplet applet;
    private final NRIterativeBodyPartAgent arm1;
    private final NRIterativeBodyPartAgent arm2;
    public float neckArmDistance = 0;

    private final Vec neck = new Vec(0, 0);
    private final Vec neckGoal = new Vec(neck);

    private List<Vec> path = new ArrayList<>();
    private int nextMilestone = 0;
    private int state = 0;

    public TwoArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeBodyPartAgent(applet, 2);
        this.arm2 = new NRIterativeBodyPartAgent(applet, 2);
    }

    public void spawn(Vec neck, float neckToArmDistance, List<Vec> path, Vec armLengths) {
        this.neckArmDistance = neckToArmDistance;
        this.neck.headSet(neck);
        this.neckGoal.headSet(neck);
        this.arm1.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
        this.arm2.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.75f, PApplet.PI * 0.25f));
        this.path = new ArrayList<>(path);
        this.nextMilestone = 0;
        this.state = 0;
    }

    public boolean update(float dt) {
        if (isPaused) {
            return false;
        }
        if (nextMilestone >= path.size()) {
            return false;
        }
        boolean shouldPlayClickSound = false;
        switch (state) {
            case 0:
                if (!arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (nextMilestone + 1 < path.size()
                        && path.get(nextMilestone + 1).get(1) <= path.get(nextMilestone).get(1)
                        && path.get(nextMilestone + 1).minus(neck).norm() < arm1.totalArmLength()) {
                    nextMilestone = nextMilestone + 1;
                }
                arm1.setGoal(path.get(nextMilestone));
                state++;
                break;
            case 1:
                if (arm1.update(dt, MIN_LIMB_SPEED)) {
                    state++;
                    shouldPlayClickSound = true;
                }
                break;
            case 2:
                Vec neckToGoal = path.get(nextMilestone).minus(neck);
                float neckToGoalDist = neckToGoal.norm();
                neckToGoal.normalizeInPlace().scaleInPlace(neckToGoalDist - neckArmDistance);
                neckGoal.headSet(neck.plus(neckToGoal));
                if (arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (arm2.isStraight()) {
                    arm2.switchPivot();
                }
                state++;
                break;
            case 3:
                if (Vec.dist(neck, neckGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK) {
                    state++;
                }
                neck.plusInPlace(neckGoal.minus(neck).scaleInPlace(NECK_SPEED));
                arm1.setGoal(neck);
                arm2.setGoal(neck);
                for (int i = 0; i < NECK_SYNC_ITERATIONS; i++) {
                    boolean arm1Ok = arm1.update(dt, MIN_LIMB_SPEED);
                    boolean arm2Ok = arm2.update(dt, MIN_LIMB_SPEED);
                    if (arm1Ok && arm2Ok) {
                        break;
                    }
                }
                break;
            case 4:
                if (!arm2.isStraight()) {
                    arm2.switchPivot();
                }
                if (nextMilestone + 1 < path.size()
                        && path.get(nextMilestone + 1).get(1) <= path.get(nextMilestone).get(1)
                        && path.get(nextMilestone + 1).minus(neck).norm() < arm2.totalArmLength()) {
                    nextMilestone = nextMilestone + 1;
                }
                arm2.setGoal(path.get(nextMilestone));
                state++;
                break;
            case 5:
                if (arm2.update(dt, MIN_LIMB_SPEED)) {
                    shouldPlayClickSound = true;
                    state++;
                }
                break;
            case 6:
                Vec neckToBelowMilestone = new Vec(path.get(nextMilestone).get(0), path.get(nextMilestone).get(1) + neckArmDistance);
                neckGoal.headSet(neckToBelowMilestone);
                if (arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (arm2.isStraight()) {
                    arm2.switchPivot();
                }
                state++;
                break;
            case 7:
                if (Vec.dist(neck, neckGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK) {
                    nextMilestone++;
                    state = 0;
                }
                neck.plusInPlace(neckGoal.minus(neck).scaleInPlace(NECK_SPEED));
                arm1.setGoal(neck);
                arm2.setGoal(neck);
                for (int i = 0; i < NECK_SYNC_ITERATIONS; i++) {
                    boolean arm1Ok = arm1.update(dt, MIN_LIMB_SPEED);
                    boolean arm2Ok = arm2.update(dt, MIN_LIMB_SPEED);
                    if (arm1Ok && arm2Ok) {
                        break;
                    }
                }
                break;
        }
        return shouldPlayClickSound;
    }

    public void draw() {
        // path
        if (DRAW_PATH) {
            applet.stroke(1);
            for (int i = 0; i < path.size() - 1; i++) {
                Vec v1 = path.get(i);
                Vec v2 = path.get(i + 1);
                applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
            }
        }
        applet.noStroke();
        applet.fill(1);
        for (Vec v : path) {
            applet.pushMatrix();
            applet.translate(0, v.get(1), v.get(0));
            applet.box(1);
            applet.popMatrix();
        }
        applet.noStroke();

        // Next milestone
        if (nextMilestone < path.size()) {
            applet.noStroke();
            applet.fill(1, 0, 0);
            applet.pushMatrix();
            applet.translate(0, path.get(nextMilestone).get(1), path.get(nextMilestone).get(0));
            applet.box(1);
            applet.popMatrix();
        }

        // Neck
        applet.noStroke();
        applet.fill(1, 1, 0);
        applet.pushMatrix();
        applet.translate(0, neck.get(1), neck.get(0));
        applet.box(3);
        applet.popMatrix();

        // Body
        applet.stroke(1);
        applet.strokeWeight(4);
        applet.line(0, neck.get(1), neck.get(0), 0, neck.get(1) + 20, neck.get(0));
        applet.strokeWeight(1);

        arm1.draw();
        arm2.draw();
    }
}
