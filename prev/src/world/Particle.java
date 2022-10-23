package world;

import math.Vec;
import processing.core.PApplet;

public class Particle {
    Vec position;
    Vec velocity;
    Vec acceleration;
    float lifetime;
    protected PApplet applet;

    public Particle(Vec position, PApplet applet) {
        this.applet = applet;
        this.position = position.plus(
                new Vec(applet.random(5.0f), applet.random(3.0f))
        );
        this.velocity = new Vec(applet.random(-3.0f, 3.0f), applet.random(-5.0f, 0));
        this.acceleration = new Vec(0, 10);
        this.lifetime = 255.0f;
    }

    public void update(float dt) {
    	if(position.get(1) > 150) {
    		position.set(1, 150);
    		velocity.set(1, -0.5f * velocity.get(1));
    		return;
    	}
        velocity.plusInPlace(acceleration.scale(dt));
        position.plusInPlace(velocity.scale(dt));
        lifetime -= 0.5f;
    }

    public boolean isAlive() {
        return (lifetime > 0);
    }

    public void draw() {
        applet.stroke(171.0f / 255, 174.0f / 255, 175.0f / 255);
        applet.point(position.get(0), position.get(1));
    }
}
