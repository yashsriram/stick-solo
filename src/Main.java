import camera.QueasyCam;
import math.Vec;
import processing.core.PApplet;
import robot.acting.RRAgent;
import robot.planning.prm.PRM;

import java.util.ArrayList;
import java.util.List;

public class Main extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    private static final int SIZE = 100;
    private static String SEARCH_ALGORITHM = "";

    private static final Vec MIN_CORNER = new Vec(new float[]{0, -SIZE, -SIZE});
    private static final Vec MAX_CORNER = new Vec(new float[]{0, SIZE, SIZE});
    private static final Vec START_POSITION = new Vec(new float[]{0, SIZE * (9f / 10), SIZE * (-9f / 10)});
    private static final Vec FINISH_POSITION = new Vec(new float[]{0, SIZE * (-9f / 10), SIZE * (9f / 10)});

    QueasyCam cam;
    PRM prm;
    RRAgent rrAgent;
    Vec goal = new Vec(new float[]{0, 0});

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        prm = new PRM(this);
        int numMilestones = 500;
        int numEdges = prm.grow(numMilestones, MIN_CORNER, MAX_CORNER, 10 + 7);
        PApplet.println("# milestones : " + numMilestones + " # edges : " + numEdges);
        List<Vec> path = prm.weightedAStar(START_POSITION, FINISH_POSITION, 25, 1.5f);
        List<Vec> cleanedPath = new ArrayList<>();
        for (Vec v : path) {
            cleanedPath.add(new Vec(new float[]{v.get(2), v.get(1)}));
        }
        rrAgent = new RRAgent(this, 10, 15, 0, -90, cleanedPath.get(0).get(0) - 10, cleanedPath.get(0).get(1) + 10, cleanedPath);
    }

    public void draw() {
        // Reset
        background(0);

        // Update
        for (int i = 0; i < 20; i++) {
            rrAgent.update(0.002f);
        }

        // Draw
        rrAgent.draw();
        prm.draw();

        surface.setTitle("Processing:" + " FPS: " + frameRate);
    }

    @Override
    public void keyPressed() {
        if (key == 'c') {
            cam.controllable = !cam.controllable;
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"Main"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
