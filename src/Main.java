import camera.QueasyCam;
import org.ejml.simple.SimpleMatrix;
import processing.core.PApplet;

public class Main extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;

    QueasyCam cam;
    NLinkAgent nLinkAgent;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        nLinkAgent = new NLinkAgent(this, 0, 0);
        nLinkAgent.addLink(PI / 6, 10);
        nLinkAgent.addLink(PI / 6, 10);
    }

    public void draw() {
        background(0);
        nLinkAgent.draw();
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
