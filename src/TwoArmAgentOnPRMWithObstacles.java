import camera.QueasyCam;
import ddf.minim.AudioPlayer;
import ddf.minim.Minim;
import math.Vec;
import processing.core.PApplet;
import robot.acting.TwoArmAgent;
import robot.planning.prm.PRM;
import robot.sensing.LineSegmentObstacle;

import java.util.List;

public class TwoArmAgentOnPRMWithObstacles extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";
    private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(-SIZE * 0.9f, SIZE * 0.9f);
    private static final Vec GOAL_POSITION = new Vec(SIZE * 0.9f, -SIZE * 0.9f);
    private static final float L1 = 12;
    private static final float L2 = 12;
    private static final float MAX_EDGE_LEN = 8;
    private static final float MIN_EDGE_LEN = 2;
    private static final int NUM_MILESTONES = 2000;
    private static final float NECK_ARM_DIST = 12;

    QueasyCam cam;
    Minim minim;
    AudioPlayer player;
    TwoArmAgent twoArmAgent;
    PRM prm;
    LineSegmentObstacle obs1;
    LineSegmentObstacle obs2;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        minim = new Minim(this);
        player = minim.loadFile("sounds/snapping-fingers.mp3");
        twoArmAgent = new TwoArmAgent(this);
        prm = new PRM(this);
        int numEdges = prm.grow(NUM_MILESTONES, MIN_CORNER, MAX_CORNER, MIN_EDGE_LEN, MAX_EDGE_LEN);
        PApplet.println("# milestones : " + NUM_MILESTONES + " # edges : " + numEdges);
        obs1 = new LineSegmentObstacle(this, new Vec(-10, 0), new Vec(10, 0), new Vec(1, 0, 1));
        obs2 = new LineSegmentObstacle(this, new Vec(11, 0), new Vec(20, 0), new Vec(1, 0, 1));
        PApplet.println(obs1.doesIntersect(obs2.e1, obs2.e2));
    }

    public void draw() {
        // Reset
        background(0);

        // Update
//        for (int i = 0; i < 15; i++) {
//            boolean playSound = twoArmAgent.update(0.00001f);
//            if (playSound) {
//                player.play(0);
//            }
//        }

        // Draw
//        twoArmAgent.draw();
//        prm.draw();
        obs1.draw();
        obs2.draw();

        surface.setTitle("Processing:"
                + " FPS: " + (int) frameRate
                + " Search: " + SEARCH_ALGORITHM
        );
    }

    @Override
    public void keyPressed() {
        if (key == 'c') {
            cam.controllable = !cam.controllable;
        }
        if (key == 'p') {
            twoArmAgent.isPaused = !twoArmAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == 'h') {
            TwoArmAgent.DRAW_PATH = !TwoArmAgent.DRAW_PATH;
        }
        if (key == '1') {
            List<Vec> path = prm.dfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            twoArmAgent.spawn(START_POSITION.plus(new Vec(0, NECK_ARM_DIST)), NECK_ARM_DIST, path, new Vec(L1, L2));
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            List<Vec> path = prm.bfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            twoArmAgent.spawn(START_POSITION.plus(new Vec(0, NECK_ARM_DIST)), NECK_ARM_DIST, path, new Vec(L1, L2));
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            List<Vec> path = prm.ucs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            twoArmAgent.spawn(START_POSITION.plus(new Vec(0, NECK_ARM_DIST)), NECK_ARM_DIST, path, new Vec(L1, L2));
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            List<Vec> path = prm.aStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            twoArmAgent.spawn(START_POSITION.plus(new Vec(0, NECK_ARM_DIST)), NECK_ARM_DIST, path, new Vec(L1, L2));
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            List<Vec> path = prm.weightedAStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, 1.5f);
            twoArmAgent.spawn(START_POSITION.plus(new Vec(0, NECK_ARM_DIST)), NECK_ARM_DIST, path, new Vec(L1, L2));
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"TwoArmAgentOnPRMWithObstacles"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
