package world;
import java.util.ArrayList;
import java.util.List;

import math.Vec;
import processing.core.PApplet;

public class Stone {
	Vec position;
	Vec velocity;
	Vec acceleration;
	int numParticles = 500;
	List<Particle> dust;
	boolean dustEnded = false;
	private PApplet applet;
	
	public void update(float dt) {
		if(position.get(1) < 150) {
			velocity.plusInPlace(acceleration.scale(dt));
			position.plusInPlace(velocity.scale(dt));
		}
		addParticles();
		updateParticles(dt);
	}
	
	private void addParticles() {
		if(dust.size() < numParticles && !dustEnded) {
			for(int i=0; i<10; i++) {
				dust.add(new Particle(position, applet));
			}
		}
	}
	
	private void updateParticles(float dt) {
		boolean allDead = true;
		for(Particle particle: dust) {
			if(particle.isAlive()) {
				particle.update(dt);
				allDead = false;
			}
		}
		if(allDead) { endDust();}
	}

	private void endDust() {
		this.dust = new ArrayList<>();
		this.dustEnded = true;
	}
	
	public void draw() {
		// Draw stone
		applet.pushMatrix();
		applet.translate(position.get(0), position.get(1));
		applet.fill(171.0f/255,174.0f/255,175.0f/255);
		applet.sphere(3.0f);
		applet.popMatrix();
		
		// Draw dust
		applet.pushMatrix();
		for(Particle particle : dust) {
			particle.draw();
		}
		applet.popMatrix();
	}

	public Stone(Vec position, PApplet applet) {
		this.position = new Vec(position);
		this.velocity = new Vec(0,0);
		this.acceleration = new Vec(0, 10);
		this.applet = applet;
		this.dust = new ArrayList<>();
	}

}
