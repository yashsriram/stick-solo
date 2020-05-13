package demos.fourarms;

import camera.QueasyCam;
import ddf.minim.AudioPlayer;
import ddf.minim.Minim;
import math.Vec;
import processing.core.PApplet;
import robot.acting.FourArmAgent;
import robot.acting.NRIterativeBodyPartAgent;
import robot.planning.prm.Milestone;
import robot.planning.prm.PRM;
import robot.sensing.PositionConfigurationSpace;
import world.Leaf;
import world.World;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class FourArmAgentOnPRMWithContext extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";
    private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(-SIZE * 0.9f, SIZE * 0.8f);
    private static final Vec GOAL_POSITION = new Vec(SIZE * 0.9f, -SIZE * 0.9f);
    private static final float L1 = 10;
    private static final float L2 = 10;
    private static final float MAX_EDGE_LEN = 7;
    private static final float MIN_EDGE_LEN = 3;
    private static final int NUM_MILESTONES = 2000;
    private static final float NECK_ARM_DIST = 8;
    private static final Vec NECK = START_POSITION.plus(new Vec(0, NECK_ARM_DIST));
    private static final Vec TAIL = START_POSITION.plus(new Vec(0, NECK_ARM_DIST + 10));
    public static final float INITIAL_ENERGY = 100f;
    public static final Vec WIND = new Vec(30, 0);
    public static final int NUM_LEAVES = 400 ;

    QueasyCam cam;
    Minim minim;
    AudioPlayer player;
    FourArmAgent fourArmAgent;
    PositionConfigurationSpace cs;
    PRM prm;
    List<Leaf> leaves ;
    private boolean pathChangeProcessing = false;
    private World world;


    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        this.randomSeed(0);
        cam = new QueasyCam(this);
        minim = new Minim(this);
        player = minim.loadFile("sounds/snapping-fingers.mp3");
        fourArmAgent = new FourArmAgent(this);
        cs = new PositionConfigurationSpace(this, List.of());
        world = new World(this, MIN_CORNER, MAX_CORNER, cs.obstacles);
        prm = new PRM(this);
        int numEdges = prm.grow(NUM_MILESTONES, MIN_CORNER, MAX_CORNER, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
        PApplet.println("# milestones : " + NUM_MILESTONES + " # edges : " + numEdges);
        NRIterativeBodyPartAgent.METHOD = NRIterativeBodyPartAgent.IKMethod.JACOBIAN_TRANSPOSE;
        leaves = new ArrayList<>();
        for(int i = 0 ; i < 50 ; i++){
            Vec p = new Vec(SIZE*random(-1, 0), SIZE*random(-1, 0));
            Vec v = new Vec(random(0, 1), random(1, 2));
            float l = random(500, 1000) ;
            leaves.add(new Leaf(p,v,5, l, this));
        }
    }

    public void draw() {
        // Reset
        background(World.SKY_COLOR);
    	lights();
        directionalLight(1f,1f,1f, 1, 1, 0);   // light from above
        
        float startTime = millis();
        // Update
        for (int i = 0; i < 15; i++) {
            this.pathChangeProcessing = fourArmAgent.switchPath;
            if (!this.pathChangeProcessing) {
                // While it's already changing path, don't do any replanning
                if (fourArmAgent.doesIntersect(cs)) {
                    replan();
                }
                checkSlippery();
            }
            world.update(0.01f);
            boolean playSound = fourArmAgent.update(0.001f);
            if (playSound) {
                player.play(0);
            }
        }
        // update leaves
        for(Leaf l : leaves){
            l.update(0.1f, SIZE, WIND);
        }
        float updateTime = millis();

        // Draw
        world.draw();
        fourArmAgent.draw();
        prm.draw();
        
        float drawTime = millis();

        // Draw Energy bar
        float energy = fourArmAgent.energy;
        beginShape();
        stroke(0, 255, 0);
        fill(0, 255, 0);
        float ht = energy / INITIAL_ENERGY;
        vertex(0, SIZE * (0.9f - 2 * ht / 10), SIZE * 0.8f);
        vertex(0, SIZE * (0.9f - 2 * ht / 10), SIZE * 0.9f);
        vertex(0, SIZE * 0.9f, SIZE * 0.9f);
        vertex(0, SIZE * 0.9f, SIZE * 0.8f);
        endShape();

        // Draw leaves
        for(Leaf l : leaves){
            l.draw();
        }

        if(leaves.size() < NUM_LEAVES){
            for(int i = 0 ; i < 10 ; i++){
                Vec p = new Vec(SIZE*random(-2, -1), SIZE*random(-1, 0));
                Vec v = new Vec(random(0, 1), random(0, 1));
                float l = random(200, 300) ;
                leaves.add(new Leaf(p,v,5, l, this));
            }
        }

        surface.setTitle("Processing:"
                + " FPS: " + (int) frameRate
                + " Search: " + SEARCH_ALGORITHM
        );
        print("FPS:"+frameRate+" Update time:"+(updateTime-startTime)+" Draw time:"+(drawTime-updateTime)+"\n");
    }

    

    private void checkSlippery() {
        List<Milestone> milestones = fourArmAgent.getMilestones();
        if (milestones.size() <= 0) {
            return;
        }
        Milestone milestone = milestones.get(0);
        if (milestone.slippery) {
            world.spawnStones(milestone.position);
            prm.removeMilestones(new ArrayList<>(Arrays.asList(milestone)));
            replan();
        }
    }
    
    void replan() {
        if (pathChangeProcessing) {
            return;
        }
        if (!fourArmAgent.goalReached()) {
            prm.removeMilestones(fourArmAgent.getMilestones());
            List<Milestone> path = prm.aStar(fourArmAgent.neck, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            fourArmAgent.setPath(path);
        }
    }

    @Override
    public void keyPressed() {
        if (key == 'c') {
            cam.controllable = !cam.controllable;
        }
        if (key == 'p') {
            fourArmAgent.isPaused = !fourArmAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == 'h') {
            FourArmAgent.DRAW_PATH = !FourArmAgent.DRAW_PATH;
        }
        if (key == '1') {
            List<Milestone> path = prm.dfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            fourArmAgent.spawn(NECK, TAIL, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            List<Milestone> path = prm.bfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            fourArmAgent.spawn(NECK, TAIL, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            List<Milestone> path = prm.ucs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            fourArmAgent.spawn(NECK, TAIL, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            List<Milestone> path = prm.aStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            fourArmAgent.spawn(NECK, TAIL, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            List<Milestone> path = prm.weightedAStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs, 1.5f);
            fourArmAgent.spawn(NECK, TAIL, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.fourarms.FourArmAgentOnPRMWithContext"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
