package world;

import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;

public class WaterParticle extends Particle {

	private float size = 5.0f;
	private Vec startPoint;

	public WaterParticle(Vec position, PApplet applet) {
		super(position, applet);
		this.startPoint = new Vec(this.position);
		init(position);
	}
	
	public void init(Vec position) {
		this.position = new Vec(
				position.get(0)+applet.random(10.0f),
				position.get(1)+applet.random(3.0f),
				applet.random(5.0f)
		);
        this.velocity = new Vec(applet.random(-10.0f, 0), applet.random(-5.0f, 0), applet.random(0,10.0f));
        this.acceleration = new Vec(0, 10, 0);
        this.lifetime = 350.0f;
	}

	public void draw(PImage waterTexture) {
//		applet.tint(1,1,1);
		applet.fill(1,1,1);
		float x = position.get(0), x_opp = x+size;
		float y = position.get(1), y_opp = y+size;
		float z = position.get(2), z_opp = z+size;
//		applet.beginShape(PApplet.QUAD_STRIP);
//		applet.texture(waterTexture);
//		applet.textureMode(PApplet.NORMAL);
//		applet.vertex(x,y,z,0,0);
//	    applet.vertex(x_opp,y_opp,z,0,1);
//	    applet.vertex(x,y,z_opp,1,0);
//	    applet.vertex(x_opp,y_opp,z_opp,1,1);
//	    applet.endShape();
//		PApplet.println("particle position:"+x+":"+y);
		applet.pushMatrix();
		applet.translate(x, y, z);
		applet.rotateY(-PApplet.PI/4);
		applet.ellipse(0, 0, size, size);
		applet.popMatrix();
	}
	
	public void respawn() {
		init(this.startPoint);
	}

}
