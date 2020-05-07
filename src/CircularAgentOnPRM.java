import camera.QueasyCam;
import math.Vec;
import processing.core.PApplet;
import robot.acting.CircularAgent;
import robot.planning.prm.PRM;

public class CircularAgentOnPRM extends PApplet {
    private static final int WIDTH = 800;
    private static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";

    private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(SIZE * (-9f / 10), SIZE * (9f / 10));
    private static final Vec GOAL_POSITION = new Vec(SIZE * (9f / 10), SIZE * (-9f / 10));
    private static final float MAX_EDGE_LEN = 7;

    PRM prm;
    QueasyCam cam;
    CircularAgent circularAgent;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);

        cam = new QueasyCam(this);
        prm = new PRM(this);
        int numMilestones = 2000;
        int numEdges = prm.grow(numMilestones, MIN_CORNER, MAX_CORNER, 0, MAX_EDGE_LEN);
        PApplet.println("# milestones : " + numMilestones + " # edges : " + numEdges);
        circularAgent = new CircularAgent(this, START_POSITION, 3, 10, new Vec(1, 1, 1));
    }

    public void draw() {
        if (keyPressed) {
            if (keyCode == RIGHT) {
                circularAgent.stepForward();
            }
            if (keyCode == LEFT) {
                circularAgent.stepBackward();
            }
        }
        long start = millis();
        // update
        circularAgent.update(0.1f);
        long update = millis();
        // draw
        background(0);
        // agent
        circularAgent.draw();
        // graph
        prm.draw();
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM);
    }

    public void keyPressed() {
        if (key == 'p') {
            circularAgent.isPaused = !circularAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == '1') {
            circularAgent.spawn(START_POSITION, prm.dfs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            circularAgent.spawn(START_POSITION, prm.bfs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            circularAgent.spawn(START_POSITION, prm.ucs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            circularAgent.spawn(START_POSITION, prm.aStar(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            circularAgent.spawn(START_POSITION, prm.weightedAStar(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, 1.5f));
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"CircularAgentOnPRM"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
