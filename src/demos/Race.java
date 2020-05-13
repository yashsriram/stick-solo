package demos;

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

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Race extends PApplet{
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
    public static final Vec WIND = new Vec(30, 0);
    public static final float INITIAL_ENERGY = 50f;
    public static final int NUM_AGENTS = 3;

    QueasyCam cam;
    Minim minim;
    AudioPlayer player;
    AudioPlayer rocksAudio;
    AudioPlayer wind;
    List<FourArmAgent> agents ;
    PositionConfigurationSpace cs;
    PRM prm;
    List<Leaf> leaves ;
    private boolean pathChangeProcessing = false;


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
        rocksAudio = minim.loadFile("sounds/rock-debris-fall.mp3");
        wind = minim.loadFile("sounds/wind01.mp3");
        agents = new ArrayList<>() ;
        for(int i =0 ; i < NUM_AGENTS; i++){
            agents.add(new FourArmAgent(this));
        }
        cs = new PositionConfigurationSpace(this, List.of());
        prm = new PRM(this);
        int numEdges = prm.grow(NUM_MILESTONES, MIN_CORNER, MAX_CORNER, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
        PApplet.println("# milestones : " + NUM_MILESTONES + " # edges : " + numEdges);
        NRIterativeBodyPartAgent.METHOD = NRIterativeBodyPartAgent.IKMethod.JACOBIAN_TRANSPOSE;
        leaves = new ArrayList<>();
        for(int i = 0 ; i < 50 ; i++){
            Vec p = new Vec(SIZE*random(-1, 0), SIZE*random(-1, 0));
            Vec v = new Vec(random(0, 1), random(2, 4));
            float l = random(500, 1000) ;
            leaves.add(new Leaf(p,v,5, l, this));
        }
        wind.loop();
    }

    public void draw() {
        // Reset
        background(0);

        // Update
        for (FourArmAgent fourArmAgent : agents) {
            for (int i = 0; i < 15; i++) {
                fourArmAgent.checkCollisionWithAgent(agents);
                this.pathChangeProcessing = fourArmAgent.switchPath;
                if (!this.pathChangeProcessing) {
                    // While it's already changing path, don't do any replanning
                    if (fourArmAgent.doesIntersect(cs)) {
                        replan(fourArmAgent);
                    }
                    checkSlippery();
                }
                boolean playSound = fourArmAgent.update(0.00001f, WIND);
                if (playSound) {
                    player.play(0);
                }

            }
        }

        for(Leaf l : leaves){
            l.update(0.1f, SIZE, WIND);
        }

        // Draw
        int k = 0 ;
        for(FourArmAgent agent : agents){ ;
            agent.draw();
        }
        prm.draw();

        // Draw leaves
        for(Leaf l : leaves){
            l.draw();
        }

        for(FourArmAgent agent :agents){
            if(agent.goalReached()){
                pauseAll();
            }
        }


        surface.setTitle("Processing:"
                + " FPS: " + (int) frameRate
                + " Search: " + SEARCH_ALGORITHM
        );
    }

    private void pauseAll() {
        for(FourArmAgent agent: agents){
            agent.isPaused = true;
        }
    }

    private void checkSlippery() {
        for(FourArmAgent agent : agents){
            List<Milestone> milestones = agent.getMilestones();
            if (milestones.size() <= 0) {
                return;
            }
            Milestone milestone = milestones.get(0);
            if (milestone.slippery) {
                rocksAudio.play(10);
                prm.removeMilestones(new ArrayList<>(Arrays.asList(milestone)));
                replan(agent);
            }
        }
    }

    void replan(FourArmAgent agent) {
        if (pathChangeProcessing) {
            return;
        }
        if (!agent.goalReached()) {
            prm.removeMilestones(agent.getMilestones());
            List<Milestone> path = prm.aStar(agent.neck, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            agent.setPath(path);
        }
    }

    @Override
    public void keyPressed() {
        if (key == 'c') {
            cam.controllable = !cam.controllable;
        }
        if (key == 'p') {
            for(FourArmAgent agent : agents){
                agent.isPaused = !agent.isPaused;
            }
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
            for(int j = 0 ; j < agents.size(); j++){
                FourArmAgent agent = agents.get(j) ;
                Vec start = getStartPositions(agents.size(), j) ;
                Vec neck =  start.plus(new Vec(0, NECK_ARM_DIST));
                Vec tail = start.plus(new Vec(0, NECK_ARM_DIST + 10));
                List<Milestone> path = prm.dfs(start, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
                agent.spawn(neck, tail, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            }
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            for(int j = 0 ; j < agents.size(); j++){
                FourArmAgent agent = agents.get(j) ;
                Vec start = getStartPositions(agents.size(), j) ;
                Vec neck =  start.plus(new Vec(0, NECK_ARM_DIST));
                Vec tail = start.plus(new Vec(0, NECK_ARM_DIST + 10));
                List<Milestone> path = prm.bfs(start, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
                agent.spawn(neck, tail, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            }
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            for(int j = 0 ; j < agents.size(); j++){
                FourArmAgent agent = agents.get(j) ;
                Vec start = getStartPositions(agents.size(), j) ;
                Vec neck =  start.plus(new Vec(0, NECK_ARM_DIST));
                Vec tail = start.plus(new Vec(0, NECK_ARM_DIST + 10));
                List<Milestone> path = prm.ucs(start, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
                agent.spawn(neck, tail, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            }
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            for(int j = 0 ; j < agents.size(); j++){
                FourArmAgent agent = agents.get(j) ;
                Vec start = getStartPositions(agents.size(), j) ;
                System.out.println(start);
                Vec neck =  start.plus(new Vec(0, NECK_ARM_DIST));
                Vec tail = start.plus(new Vec(0, NECK_ARM_DIST + 10));
                List<Milestone> path = prm.aStar(start, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
                agent.spawn(neck, tail, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            }
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            for(int j = 0 ; j < agents.size(); j++){
                FourArmAgent agent = agents.get(j) ;
                Vec start = getStartPositions(agents.size(), j) ;
                Vec neck =  start.plus(new Vec(0, NECK_ARM_DIST));
                Vec tail = start.plus(new Vec(0, NECK_ARM_DIST + 10));
                List<Milestone> path = prm.weightedAStar(start, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs, 1.5f);
                agent.spawn(neck, tail, NECK_ARM_DIST, path, new Vec(L1, L2), INITIAL_ENERGY);
            }
            SEARCH_ALGORITHM = "weighted A*";
        }
        if (key == 'x'){
            WIND.headSet(WIND.get(0)+10f, WIND.get(1));
        }
        if (key == 'v'){
            WIND.headSet(WIND.get(0)-10f, WIND.get(1));
        }
    }

    private Vec getStartPositions(int size, int j) {
        float partition_size = 2f/(float)size ;
        System.out.println(partition_size);
        System.out.println(j);
        Vec start = new Vec(SIZE*(-1 + j*partition_size + random(0, partition_size)), SIZE * 0.8f) ;
        return start;
    }


    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.Race"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
