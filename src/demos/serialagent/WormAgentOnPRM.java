package demos.serialagent;

import camera.QueasyCam;
import ddf.minim.AudioPlayer;
import ddf.minim.Minim;
import math.Vec;
import processing.core.PApplet;
import robot.acting.NRIterativeWormAgent;
import robot.planning.prm.Milestone;
import robot.planning.prm.PRM;
import robot.sensing.CircleObstacle;
import robot.sensing.LineSegmentObstacle;
import robot.sensing.PositionConfigurationSpace;

import java.util.List;

public class WormAgentOnPRM extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";

    private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(SIZE * (-9f / 10), SIZE * (9f / 10));
    private static final Vec GOAL_POSITION = new Vec(SIZE * (9f / 10), SIZE * (-9f / 10));
    private static final float MAX_EDGE_LEN = 5;
    private static final float MIN_EDGE_LEN = 0;
    private static final int NUM_MILESTONES = 4000;

    QueasyCam cam;

    Minim minim;
    AudioPlayer player;
    NRIterativeWormAgent nrIterativeWormAgent;
    PositionConfigurationSpace cs;
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
        nrIterativeWormAgent = new NRIterativeWormAgent(this, 10);
        cs = new PositionConfigurationSpace(this, List.of(
                new LineSegmentObstacle(this, new Vec(0, 20), new Vec(0, SIZE), new Vec(1, 0, 1)),
                new LineSegmentObstacle(this, new Vec(0, 20), new Vec(-50, SIZE / 2f), new Vec(1, 0, 1)),
                new LineSegmentObstacle(this, new Vec(0, SIZE), new Vec(-50, SIZE / 2f), new Vec(1, 0, 1)),
                new LineSegmentObstacle(this, new Vec(-SIZE, -20), new Vec(SIZE * 0.4f, -20), new Vec(1, 0, 1)),
                new CircleObstacle(this, new Vec(SIZE * 0.4f, -20), 10, new Vec(1, 0, 1)),
                new LineSegmentObstacle(this, new Vec(-0.4f * SIZE, -60), new Vec(SIZE, -60), new Vec(1, 0, 1)),
                new CircleObstacle(this, new Vec(-0.4f * SIZE, -60), 10, new Vec(1, 0, 1))
        ));
        prm = new PRM(this);
        prm.margin = 10;
        int numEdges = prm.grow(NUM_MILESTONES, MIN_CORNER, MAX_CORNER, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
        PApplet.println("# milestones : " + NUM_MILESTONES + " # edges : " + numEdges);
    }

    public void draw() {
        // Reset
        background(0);

        // Update
        for (int i = 0; i < 15; i++) {
            boolean isPivotSwitched = nrIterativeWormAgent.update(0.00003f);
            if (isPivotSwitched) {
                player.play(0);
            }
        }

        // Draw
        nrIterativeWormAgent.draw();
        prm.draw();
        cs.draw();

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
            nrIterativeWormAgent.isPaused = !nrIterativeWormAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == '1') {
            List<Milestone> path = prm.dfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            nrIterativeWormAgent.spawn(path, new Vec(2, 2, 2, 2, 2, 2, 2, 2, 2, 2), new Vec(0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 3, 1);
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            List<Milestone> path = prm.bfs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            nrIterativeWormAgent.spawn(path, new Vec(2, 2, 2, 2, 2, 2, 2, 2, 2, 2), new Vec(0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 3, 1);
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            List<Milestone> path = prm.ucs(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            nrIterativeWormAgent.spawn(path, new Vec(2, 2, 2, 2, 2, 2, 2, 2, 2, 2), new Vec(0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 3, 1);
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            List<Milestone> path = prm.aStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs);
            nrIterativeWormAgent.spawn(path, new Vec(2, 2, 2, 2, 2, 2, 2, 2, 2, 2), new Vec(0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 3, 1);
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            List<Milestone> path = prm.weightedAStar(START_POSITION, GOAL_POSITION, MIN_EDGE_LEN, MAX_EDGE_LEN, cs, 1.5f);
            nrIterativeWormAgent.spawn(path, new Vec(2, 2, 2, 2, 2, 2, 2, 2, 2, 2), new Vec(0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 3, 1);
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.serialagent.WormAgentOnPRM"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
