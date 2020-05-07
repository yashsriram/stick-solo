import camera.QueasyCam;
import ddf.minim.AudioPlayer;
import ddf.minim.Minim;
import math.Vec;
import processing.core.PApplet;
import robot.acting.NRIterativeAgent;
import robot.planning.prm.PRM;

import java.util.List;

public class NRJacobianAgentOnPRM extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";

    private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(SIZE * (-9f / 10), SIZE * (9f / 10));
    private static final Vec GOAL_POSITION = new Vec(SIZE * (9f / 10), SIZE * (-9f / 10));
    private static final float L1 = 5;
    private static final float L2 = 10;
    private static final float L3 = 5;
    private static final float L4 = 10;
    private static final float MAX_EDGE_LEN = L1 + L2 + L3 + L4 - 5;
    private static final float MIN_EDGE_LEN = 0;
    private static final int NUM_MILESTONES = 500;

    QueasyCam cam;
    Minim minim;
    AudioPlayer player;
    NRIterativeAgent nrIterativeAgent;
    PRM prm;

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
        nrIterativeAgent = new NRIterativeAgent(this, 4);
        prm = new PRM(this);
        int numEdges = prm.grow(NUM_MILESTONES, MIN_CORNER, MAX_CORNER, MIN_EDGE_LEN, MAX_EDGE_LEN);
        PApplet.println("# milestones : " + NUM_MILESTONES + " # edges : " + numEdges);
    }

    public void draw() {
        // Reset
        background(0);

        // Update
        for (int i = 0; i < 15; i++) {
            boolean isPivotSwitched = nrIterativeAgent.update(0.00001f);
            if (isPivotSwitched) {
                player.play(0);
            }
        }

        // Draw
        nrIterativeAgent.draw();
        prm.draw();

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
            nrIterativeAgent.isPaused = !nrIterativeAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == '1') {
            List<Vec> path = prm.dfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            nrIterativeAgent.spawn(path, new Vec(L1, L2, L3, L4), new Vec(0, 0, 0, 0));
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            List<Vec> path = prm.bfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            nrIterativeAgent.spawn(path, new Vec(L1, L2, L3, L4), new Vec(0, 0, 0, 0));
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            List<Vec> path = prm.ucs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            nrIterativeAgent.spawn(path, new Vec(L1, L2, L3, L4), new Vec(0, 0, 0, 0));
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            List<Vec> path = prm.aStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN);
            nrIterativeAgent.spawn(path, new Vec(L1, L2, L3, L4), new Vec(0, 0, 0, 0));
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            List<Vec> path = prm.weightedAStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, 1.5f);
            nrIterativeAgent.spawn(path, new Vec(L1, L2, L3, L4), new Vec(0, 0, 0, 0));
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"NRJacobianAgentOnPRM"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
