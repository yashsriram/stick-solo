package robot.acting;

import math.Vec;
import processing.core.PApplet;
import robot.planning.prm.Milestone;
import robot.sensing.PositionConfigurationSpace;

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
    public final Vec neck = new Vec(0, 0);
    public float neckArmDistance = 0;

    private List<Milestone> path = new ArrayList<>();
    private int nextMilestone = 0;
    private List<Milestone> newPath;
    public boolean switchPath = false;
    private int state = 0;
    private final Vec neckGoal = new Vec(neck);
    public NRIterativeBodyPartAgent currentlyMovingArm;
	

    public TwoArmAgent(PApplet applet) {
        this.applet = applet;
        this.arm1 = new NRIterativeBodyPartAgent(applet, 0, 2);
        this.arm2 = new NRIterativeBodyPartAgent(applet, 1, 2);
    }

    public void spawn(Vec neck, float neckToArmDistance, List<Milestone> path, Vec armLengths) {
        this.neckArmDistance = neckToArmDistance;
        this.neck.headSet(neck);
        this.neckGoal.headSet(neck);
        this.arm1.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.25f, -PApplet.PI * 0.25f));
        this.arm2.spawn(neck, new Vec(armLengths), new Vec(-PApplet.PI * 0.75f, PApplet.PI * 0.25f));
        this.path = new ArrayList<>(path);
        this.nextMilestone = 0;
        this.state = 0;
        this.currentlyMovingArm = arm1;
    }
    
    public void setPath(List<Milestone> newPath) {
    	this.newPath = new ArrayList<>(newPath);
    	this.switchPath  = true;
	}

    private void switchCurrentlyMovingArm() {
        if (currentlyMovingArm.id == arm1.id) {
            currentlyMovingArm = arm2;
        } else {
            currentlyMovingArm = arm1;
        }
    }
    
    public boolean doesIntersect(PositionConfigurationSpace cs) {
    	if(currentlyMovingArm != null) {
    		return currentlyMovingArm.doesIntersect(cs);
    	}
    	return false;
    }
    
    public List<Milestone> getMilestones(){
    	if(this.nextMilestone >= this.path.size()) {return new ArrayList<>();}
    	List<Milestone> milestones = new ArrayList<>();
    	milestones.add(this.path.get(this.nextMilestone));
    	if(this.nextMilestone < this.path.size()-1) {
    		milestones.add(this.path.get(this.nextMilestone+1));
    	}
    	return milestones; 
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
            // Set arm goal to next milestone or next + 1 milestone
            case 0:
            	if(this.switchPath) {
            		this.path = new ArrayList<>(newPath);
                	this.nextMilestone = 0;
                	this.switchPath = false;
            	}
                if (!currentlyMovingArm.isStraight()) {
                    currentlyMovingArm.switchPivot();
                }
                if (nextMilestone + 1 < path.size()
                        && path.get(nextMilestone + 1).position.get(1) <= path.get(nextMilestone).position.get(1)
                        && path.get(nextMilestone + 1).position.minus(neck).norm() < currentlyMovingArm.totalArmLength()) {
                    nextMilestone = nextMilestone + 1;
                }
                currentlyMovingArm.setGoal(path.get(nextMilestone).position);
                state++;
                break;
            // Move arm
            case 1:
                if (currentlyMovingArm.update(dt, MIN_LIMB_SPEED)) {
                    state++;
                    shouldPlayClickSound = true;
                }
                break;
            // Set neck goal to distance from the next milestone
            case 2:
                Vec neckToBelowMilestone1 = new Vec(path.get(nextMilestone).position.get(0), path.get(nextMilestone).position.get(1) + neckArmDistance);
                neckGoal.headSet(neckToBelowMilestone1);
                if (arm1.isStraight()) {
                    arm1.switchPivot();
                }
                if (arm2.isStraight()) {
                    arm2.switchPivot();
                }
                state++;
                break;
            // Move neck, i.e. move both arms simultaneously
            case 3:
                if (Vec.dist(neck, neckGoal) < NRIterativeBodyPartAgent.MILESTONE_REACHED_SLACK) {
                    nextMilestone++;
                    switchCurrentlyMovingArm();
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
                Vec v1 = path.get(i).position;
                Vec v2 = path.get(i + 1).position;
                applet.line(0, v1.get(1), v1.get(0), 0, v2.get(1), v2.get(0));
            }
        }
        applet.noStroke();
        applet.fill(1);
        for (Milestone milestone : path) {
        	Vec v = milestone.position;
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
            applet.translate(0, path.get(nextMilestone).position.get(1), path.get(nextMilestone).position.get(0));
            applet.box(1);
            applet.popMatrix();
        }

        // Neck
        applet.noStroke();
        applet.fill(1, 1, 0);
        applet.pushMatrix();
        applet.translate(0, neck.get(1), neck.get(0));
        applet.sphere(3);
        applet.popMatrix();

        // Body
        applet.stroke(1);
        applet.strokeWeight(4);
        applet.line(0, neck.get(1), neck.get(0), 0, neck.get(1) + 20, neck.get(0));
        applet.strokeWeight(1);

        arm1.draw();
        arm2.draw();
    }

	public boolean goalReached() {
		return (this.nextMilestone == this.path.size());
	}

}
