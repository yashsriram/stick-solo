import camera.QueasyCam;
import math.Vec;
import processing.core.PApplet;
import robot.acting.RRAgent;

public class Main extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;

    QueasyCam cam;
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
        rrAgent = new RRAgent(this, 50, 50, 0, -90, 0, 0);
        rrAgent.setGoalPosition(goal);
    }

    public void draw() {
        // Interaction
        if (keyPressed) {
            if (keyCode == UP) {
                goal.set(1, goal.get(1) - 1);
                rrAgent.setGoalPosition(goal);
            }
            if (keyCode == DOWN) {
                goal.set(1, goal.get(1) + 1);
                rrAgent.setGoalPosition(goal);
            }
            if (keyCode == LEFT) {
                goal.set(0, goal.get(0) - 1);
                rrAgent.setGoalPosition(goal);
            }
            if (keyCode == RIGHT) {
                goal.set(0, goal.get(0) + 1);
                rrAgent.setGoalPosition(goal);
            }
        }

        // Reset
        background(0);

        // Update
        rrAgent.update(0.01f);

        // Draw
        rrAgent.draw();
        fill(0, 1, 0);
        pushMatrix();
        translate(0, goal.get(1), goal.get(0));
        box(2);
        popMatrix();

        surface.setTitle("Processing:"  + " FPS: " + frameRate);
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
