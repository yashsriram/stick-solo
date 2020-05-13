package robot.acting;

import math.Vec;
import processing.core.PApplet;
import processing.core.PShape;
import robot.planning.prm.Milestone;
import robot.sensing.PositionConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class FourArmAgent {
    public static boolean DRAW_PATH = true;
    public static float INIT_LIMB_SPEED = 0.006f;
    public static float MIN_LIMB_SPEED = 0.006f;
    public static float NECK_SPEED = 0.01f;
    public static float BODY_LENGTH;
    public static float INITIAL_ENERGY ;
    public static float ENERGY ;
    public static float REDUCE_ENERGY = 1f;
    public static float REDUCE_SPEED = 0.00005f;
    public static float RECOVERY_RATE = 0.01f ;
    public static float NECK_SYNC_ITERATIONS = 150;
    public static float WIND_FORCE_COEFFICIENT = 0.01f;
    public static final Vec INIT_LEG_VECTOR = new Vec(-5, 5);
    public static final Vec LEG_VECTOR = new Vec(-6, 3);
    public static final float COLLISION_RADIUS = 10 ;

    public boolean isPaused = false;

    private final PApplet applet;
    private final NRIterativeBodyPartAgent arm1;
    private final NRIterativeBodyPartAgent arm2;
    private final NRIterativeBodyPartAgent arm3;
    private final NRIterativeBodyPartAgent arm4;
    public float neckArmDistance = 0;
    public float tailLegDistance = 0;

    public final Vec neck = new Vec(0, 0);
    private final Vec tail = new Vec(0, 0);
    private final Vec neckGoal = new Vec(neck);
    private final Vec tailGoal = new Vec(tail);
    private NRIterativeBodyPartAgent currentlyMovingArm;
    private NRIterativeBodyPartAgent currentlyMovingLeg;
    private Vec legVector = new Vec(0, 0) ;
    private float energy ;

    private List<Milestone> path = new ArrayList<>();
    private int nextMilestone = 0;
    private int state = 0;
    private ArrayList newPath;
    private boolean isRecharging;
    public boolean switchPath;
	private PShape neckShape;
	private PShape bodyShape;

    public FourArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeBodyPartAgent(applet, 0, 2, load("right_arm"), load("right_hand"));
        this.arm2 = new NRIterativeBodyPartAgent(applet, 1, 2, load("left_arm"), load("left_hand"));
        this.arm3 = new NRIterativeBodyPartAgent(applet, 2, 2, load("left_thigh"), load("left_leg"));
        this.arm4 = new NRIterativeBodyPartAgent(applet, 3, 2, load("right_thigh"), load("right_leg"));
//        this.neckShape = load("neck");
        this.bodyShape = load("body");
    }
    
    private PShape load(String shapeName) {
    	PShape shape = applet.loadShape("stickman/"+shapeName+".obj");
    	shape.scale(5);
    	if(shapeName.contains("arm") || shapeName.contains("hand")) {
    		shape.rotateY(PApplet.HALF_PI);
    	}
    	return shape;
    }

    public void spawn(Vec neck, Vec tail, float neckToArmDistance, List<Milestone> path, Vec armLengths, float initial_energy) {
        this.neckArmDistance = neckToArmDistance;
        this.tailLegDistance = neckToArmDistance;
        this.neck.headSet(neck);
        this.tail.headSet(tail);
        this.neckGoal.headSet(neck);
        this.tailGoal.headSet(tail);
        this.arm1.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
        this.arm2.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.95f, PApplet.PI * 0.55f));
        this.arm3.spawn(tail, new Vec(armLengths), new Vec(PApplet.PI * 2 + PApplet.PI * 0.05f, PApplet.PI * 2 + PApplet.PI * 0.55f));
        this.arm4.spawn(tail, new Vec(armLengths), new Vec(PApplet.PI * 2 + PApplet.PI * 0.95f, PApplet.PI * 2 - PApplet.PI * 0.55f));
        this.path = new ArrayList<>(path);
        this.nextMilestone = 0;
        this.state = 0;
        this.currentlyMovingArm = arm1;
        this.currentlyMovingLeg = arm4;
        BODY_LENGTH = neck.minus(tail).norm();
        INITIAL_ENERGY = initial_energy;
        ENERGY = initial_energy;
        this.energy = initial_energy;
        MIN_LIMB_SPEED = INIT_LIMB_SPEED;
        isPaused = false;
        this.isRecharging = false;
        legVector.headSet(INIT_LEG_VECTOR);
        LEG_VECTOR.headSet(INIT_LEG_VECTOR);
    }

    private void switchCurrentlyMovingLeg() {
        if (currentlyMovingLeg.id == arm3.id) {
            currentlyMovingLeg = arm4;
        } else {
            currentlyMovingLeg = arm3;
        }
    }

    private void switchCurrentlyMovingArm() {
        if (currentlyMovingArm.id == arm1.id) {
            currentlyMovingArm = arm2;
        } else {
            currentlyMovingArm = arm1;
        }
    }

    public void setPath(List<Milestone> newPath) {
        this.newPath = new ArrayList<>(newPath);
        this.switchPath = true;
    }

    public boolean doesIntersect(PositionConfigurationSpace cs) {
        if (currentlyMovingArm != null) {
            return currentlyMovingArm.doesIntersect(cs);
        }
        return false;
    }

    public boolean goalReached() {
        return (this.nextMilestone == this.path.size());
    }

    public List<Milestone> getMilestones() {
        if (this.nextMilestone >= this.path.size()
                || this.nextMilestone <= 0) {
            return new ArrayList<>();
        }
        List<Milestone> milestones = new ArrayList<>();
        milestones.add(this.path.get(this.nextMilestone - 1));
        milestones.add(this.path.get(this.nextMilestone));
        return milestones;
    }

    public boolean update(float dt){
        Vec w = new Vec(0,0);
        return update(dt, w);
    }

    public boolean update(float dt, Vec wind) {
        if (isPaused) {
            return false;
        }
        if (nextMilestone >= path.size()) {
            return false;
        }
        if (this.energy <= 5f) {
            isRecharging = true ;
        }
        if(isRecharging){
            if(this.energy < INITIAL_ENERGY){
                this.energy += RECOVERY_RATE ;
                ENERGY += RECOVERY_RATE ;
                MIN_LIMB_SPEED = INIT_LIMB_SPEED;
                return false;
            }else{
                isRecharging = false ;
                return false;
            }
        }
        boolean shouldPlayClickSound = false;
        REDUCE_ENERGY = 0.05f*this.energy;
        switch (state) {
            // Set arm1 goal to next milestone or next + 1 milestone
            case 0:
                if (!currentlyMovingArm.isStraight()) {
                    currentlyMovingArm.switchPivot();
                }
                currentlyMovingArm.setGoal(path.get(nextMilestone).position);
                state++;
                break;
            // Move arm1
            case 1:
                if (currentlyMovingArm.update(dt, MIN_LIMB_SPEED)) {
                    state++;
                    ENERGY -= REDUCE_ENERGY;
                    this.energy -= REDUCE_ENERGY;
                    MIN_LIMB_SPEED -= REDUCE_SPEED;
                    shouldPlayClickSound = true;
                }
                break;
            // Set neck goal to distance from the next milestone
            // move tail on the same line as neck
            case 2:
                Vec neckToBelowMilestone = new Vec(path.get(nextMilestone).position.get(0), path.get(nextMilestone).position.get(1) + neckArmDistance);
                neckGoal.headSet(neckToBelowMilestone);
                if (arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (arm2.isStraight()) {
                    arm2.switchPivot();
                }
                Vec tailToBelowNeck = new Vec(neckGoal.get(0), neckGoal.get(1) + BODY_LENGTH);
                tailToBelowNeck.plusInPlace(wind.scale(WIND_FORCE_COEFFICIENT));
                tailGoal.headSet(tailToBelowNeck);
                if (arm3.isStraight()) {
                    arm3.switchPivot();
                }
                if (arm4.isStraight()) {
                    arm4.switchPivot();
                }
                state++;
                break;
            // Move neck, i.e. move both arms simultaneously
            // Move tail, i.e. move both legs simultaneously
            case 3:
                if (Vec.dist(neck, neckGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK
                        && Vec.dist(tail, tailGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK) {
                    state++;
                    nextMilestone++;
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

                tail.plusInPlace(tailGoal.minus(tail).scaleInPlace(NECK_SPEED));
                arm3.setGoal(tail);
                arm4.setGoal(tail);
                for (int i = 0; i < NECK_SYNC_ITERATIONS; i++) {
                    boolean arm3Ok = arm3.update(dt, MIN_LIMB_SPEED);
                    boolean arm4Ok = arm4.update(dt, MIN_LIMB_SPEED);
                    if (arm3Ok && arm4Ok) {
                        break;
                    }
                }
                break;
            // Set arm1 goal to next milestone or next + 1 milestone
            case 4:
                if (!currentlyMovingLeg.isStraight()) {
                    currentlyMovingLeg.switchPivot();
                }
                currentlyMovingLeg.setGoal(tail.plus(this.legVector));
                switchCurrentlyMovingArm();
                state++;
                break;
            // Move leg
            case 5:
                if (currentlyMovingLeg.update(dt, MIN_LIMB_SPEED)) {
                    state = 0;
                    ENERGY -= REDUCE_ENERGY;
                    this.energy -= REDUCE_ENERGY;
                    MIN_LIMB_SPEED -= REDUCE_SPEED;
                    shouldPlayClickSound = true;
                    switchCurrentlyMovingLeg();
                    this.legVector.headSet(-this.legVector.get(0), this.legVector.get(1));
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
                Vec v1 = path.get(i).position;
                Vec v2 = path.get(i + 1).position;
                applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
            }
        }
        applet.noStroke();
        for (Milestone milestone : path) {
            Vec v = milestone.position;
            applet.pushMatrix();
            applet.fill(1);
            applet.translate(0, v.get(1), v.get(0));
            applet.box(1);
            applet.popMatrix();
        }
        applet.noStroke();

        // Next milestone
        if (nextMilestone < path.size()) {
            applet.noStroke();
            applet.pushMatrix();
            applet.fill(1, 0, 0);
            applet.translate(0, path.get(nextMilestone).position.get(1), path.get(nextMilestone).position.get(0));
            applet.box(1);
            applet.popMatrix();
        }

        // Neck
        applet.noStroke();
        applet.pushMatrix();
        applet.translate(0, neck.get(1), neck.get(0));
        applet.rotateX(PApplet.PI);
        applet.rotateY(PApplet.PI/2);
//        applet.shape(this.neckShape);
        applet.popMatrix();

        // Body
        applet.stroke(1);
//        applet.strokeWeight(4);
//        applet.line(0, neck.get(1), neck.get(0), 0, tail.get(1), tail.get(0));
        Vec color = new Vec(1-(ENERGY/INITIAL_ENERGY),(ENERGY/INITIAL_ENERGY),(ENERGY/INITIAL_ENERGY));
        applet.pushMatrix();
        applet.translate(0, tail.get(1), tail.get(0));
        applet.rotateX(PApplet.PI);
        applet.rotateY(PApplet.PI/2);
        applet.shape(this.bodyShape);
        applet.popMatrix();

//        Vec color = new Vec(1-(this.energy/INITIAL_ENERGY),(this.energy/INITIAL_ENERGY),(this.energy/INITIAL_ENERGY));
//        applet.stroke(color.get(0), color.get(1), color.get(2));
//        applet.strokeWeight(4);
//        applet.line(0, neck.get(1), neck.get(0), 0, tail.get(1), tail.get(0));

        applet.strokeWeight(1);

        // Tail
        applet.noStroke();
        applet.pushMatrix();
        applet.fill(1, 1, 0);
        applet.translate(0, tail.get(1), tail.get(0));
        applet.box(3);
        applet.popMatrix();

        arm1.draw(color);
        arm2.draw(color);
        arm3.draw(color);
        arm4.draw(color);
    }

    public void checkCollisionWithAgent(List<FourArmAgent> agents) {

    }
}
