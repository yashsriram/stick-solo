package world;

import java.util.ArrayList;
import java.util.List;

import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;

public class Waterfall {
	List<WaterParticle> particles;
	int numParticles = 5000;
	private Vec position;
	private PApplet applet;
	private boolean endWaterfall;
	
	private void addParticles() {
		if(particles.size() < numParticles && !endWaterfall) {
			for(int i=0; i<5; i++) {
				particles.add(new WaterParticle(position, applet));
			}
		}
	}
	
	private void updateParticles(float dt) {
		boolean allDead = true;
		for(WaterParticle particle: particles) {
			if(particle.isAlive()) {
				particle.update(dt);
				allDead = false;
			}else {
				particle.respawn();
			}
		}
	}
	
	public Waterfall(PApplet applet, Vec position) {
		particles = new ArrayList<>();
		this.applet = applet;
		this.position = position;
	}
	
	public void update(float dt) {
		addParticles();
		updateParticles(dt);
	}
	
	public void draw(PImage waterTexture) {
		applet.pushMatrix();
		applet.rotateY(-PApplet.PI/2);
		for(WaterParticle particle : particles) {
			particle.draw(waterTexture);
		}
		applet.popMatrix();
	}
}
