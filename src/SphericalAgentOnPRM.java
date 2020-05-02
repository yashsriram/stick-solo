import camera.QueasyCam;
import math.Vec;
import robot.acting.SphericalAgent;
import robot.planning.prm.PRM;
import processing.core.PApplet;

public class SphericalAgentOnPRM extends PApplet {
    private static final int WIDTH = 800;
    private static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";

    private static final Vec MIN_CORNER = new Vec(0, -SIZE, -SIZE);
    private static final Vec MAX_CORNER = new Vec(0, SIZE, SIZE);
    private static final Vec START_POSITION = new Vec(0, SIZE * (9f / 10), SIZE * (-9f / 10));
    private static final Vec FINISH_POSITION = new Vec(0, SIZE * (-9f / 10), SIZE * (9f / 10));
    private static final float MAX_EDGE_LEN = 7;

    PRM prm;
    QueasyCam cam;
    SphericalAgent sphericalAgent;

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
        int numEdges = prm.grow(numMilestones, MIN_CORNER, MAX_CORNER, MAX_EDGE_LEN);
        PApplet.println("# milestones : " + numMilestones + " # edges : " + numEdges);
        sphericalAgent = new SphericalAgent(this, START_POSITION, 3, 10, new Vec(1, 1, 1));
    }

    public void draw() {
        if (keyPressed) {
            if (keyCode == RIGHT) {
                sphericalAgent.stepForward();
            }
            if (keyCode == LEFT) {
                sphericalAgent.stepBackward();
            }
        }
        long start = millis();
        // update
        sphericalAgent.update(0.1f);
        long update = millis();
        // draw
        background(0);
        // agent
        sphericalAgent.draw();
        // graph
        prm.draw();
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM);
    }

    public void keyPressed() {
        if (key == 'p') {
            sphericalAgent.isPaused = !sphericalAgent.isPaused;
        }
        if (key == 'k') {
            PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
        }
        if (key == 'j') {
            PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
        }
        if (key == '1') {
            sphericalAgent.setPath(START_POSITION, prm.dfs(START_POSITION, FINISH_POSITION, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            sphericalAgent.setPath(START_POSITION, prm.bfs(START_POSITION, FINISH_POSITION, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            sphericalAgent.setPath(START_POSITION, prm.ucs(START_POSITION, FINISH_POSITION, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            sphericalAgent.setPath(START_POSITION, prm.aStar(START_POSITION, FINISH_POSITION, MAX_EDGE_LEN));
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            sphericalAgent.setPath(START_POSITION, prm.weightedAStar(START_POSITION, FINISH_POSITION, MAX_EDGE_LEN, 1.5f));
            SEARCH_ALGORITHM = "weighted A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"SphericalAgentOnPRM"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
