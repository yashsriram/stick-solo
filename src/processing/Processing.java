package processing;

import processing.core.PApplet;

public class Processing {
    public static void circleYZ(PApplet applet, float a, float b, float radius, int resolution) {
        applet.beginShape();
        for (int i = 0; i < resolution; ++i) {
            float theta = 2 * PApplet.PI / (resolution - 1) * i;
            applet.vertex(0, b + radius * PApplet.sin(theta), a + radius * PApplet.cos(theta));
        }
        applet.endShape(PApplet.CLOSE);
    }
}
