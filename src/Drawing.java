import java.util.ArrayList;
import java.util.List;

import world.Stone;
import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;
import processing.core.PShape;
import robot.sensing.CircleObstacle;
import robot.sensing.Obstacle;

public class Drawing {
	public static int SKY_COLOR = 9687551;
	private PImage wallTexture;
	private PApplet applet;
	private Vec MAX_CORNER;
	private Vec MIN_CORNER;
	private PImage landTexture;
	private PShape obstacleShape;
	private PImage canyonTexture;
	private PShape canyonShape;
	private List<Obstacle> obstacles;
	public List<Stone> stones = new ArrayList<>();
	private PImage stoneTexture;
	private PShape stoneShape;
	
	
	public void drawWorld() {
//    	drawWall();
    	drawFloor();
    	drawObstacles();
    	drawStones();
    	drawCanyon();
    	drawTracer();
    }
	
	private void drawTracer() {
		applet.pushMatrix();
		applet.fill(1,0,0);
		applet.rotateY(-PApplet.PI/2);
		applet.translate(200, -40);
		applet.sphere(4);
		applet.popMatrix();
	}

	private void drawCanyon() {
		applet.pushMatrix();
		applet.scale(40);
		applet.rotateZ(-PApplet.PI/2);
		applet.translate(20.5f,0.5f,0);
		applet.noFill();
		applet.noStroke();
		this.canyonShape.setTexture(this.canyonTexture);
		applet.shape(this.canyonShape);
		applet.popMatrix();
		
		applet.pushMatrix();
		applet.rotateY(PApplet.PI/2);
		applet.rotateZ(PApplet.PI);
		applet.scale(40);
		applet.translate(28, -3.5f, 0);
		applet.noFill();
		applet.noStroke();
		applet.shape(this.canyonShape);
		applet.translate(-15, 0, 0);
		applet.shape(this.canyonShape);
		applet.popMatrix();
		
	}
    
    private void drawWall() {
    	applet.pushMatrix();
    	applet.noStroke();
    	applet.rotateY(-PApplet.PI/2);
    	applet.beginShape();
    	applet.textureMode(PApplet.NORMAL);
    	applet.texture(this.wallTexture);
    	applet.vertex(MAX_CORNER.get(0), MIN_CORNER.get(1), 0, 0);
    	applet.vertex(MIN_CORNER.get(0), MIN_CORNER.get(1), 1, 0);
    	applet.vertex(MIN_CORNER.get(0), MAX_CORNER.get(1), 1, 1);
    	applet.vertex(MAX_CORNER.get(0), MAX_CORNER.get(1), 0, 1);
    	applet.endShape();
    	applet.popMatrix();
    	
    }
    
    private void drawFloor() {
    	applet.pushMatrix();
    	applet.rotateX(PApplet.PI/2);
    	applet.translate(0, 0, -150);
    	float startX = -500, startY = -500;
    	float sidelen = 200;
    	applet.noStroke();
    	applet.beginShape(PApplet.QUADS);
    	applet.textureMode(PApplet.NORMAL);
    	applet.texture(this.landTexture);
    	for(int i=0; i<5; i++) {
    		for(int j=0; j<5; j++) {
    	    	applet.vertex(startX+i*sidelen, startY+j*sidelen, 0, 0);
    	    	applet.vertex(startX+(i-1)*sidelen, startY+j*sidelen, 1, 0);
    	    	applet.vertex(startX+(i-1)*sidelen, startY+(j+1)*sidelen, 1, 1);
    	    	applet.vertex(startX+i*sidelen, startY+(j+1)*sidelen, 0, 1);
    	    	
    		}
    	}
    	applet.endShape();
    	applet.popMatrix();
    }
	public Drawing(PApplet applet, Vec MIN_CORNER, Vec MAX_CORNER, List<Obstacle> obstacles) {
		this.applet = applet;
		this.MAX_CORNER = MAX_CORNER;
		this.MIN_CORNER = MIN_CORNER;
		this.wallTexture = applet.loadImage("wall_texture.jpg");
		this.stoneTexture = applet.loadImage("stone_texture.jpg");
		this.landTexture = applet.loadImage("grass.png");
		this.obstacleShape = applet.loadShape("rock1.OBJ");
		this.canyonShape = applet.loadShape("mountain.obj");
		this.canyonTexture = applet.loadImage("wall_texture.jpg");
		this.obstacles = obstacles;
//		loadStoneTexture();
	}
	
	private void loadStoneTexture() {
		applet.fill(255,255,255,255);
		applet.noStroke();
		applet.noFill();
		stoneShape = applet.createShape(PApplet.SPHERE, 3);
		stoneShape.setTexture(this.stoneTexture);
	}
	
	public void drawObstacles() {
		for(Obstacle obstacle : this.obstacles) {
			if(obstacle instanceof CircleObstacle) {
				Vec position = ((CircleObstacle) obstacle).center;
				applet.pushMatrix();
				applet.shapeMode(PApplet.CENTER);
				applet.translate(-35, position.get(1)+32, position.get(0)-17);
				applet.noStroke();
				applet.noFill();
				this.obstacleShape.setTexture(this.wallTexture);
				applet.rotateZ(PApplet.PI/2);
				applet.scale(5.25f);
				applet.shape(this.obstacleShape);
				applet.popMatrix();
			}
		}
	}
	
	private void drawStones() {
		applet.pushMatrix();
		applet.rotateY(-PApplet.PI/2);
		for (Stone stone: stones) {
			stone.draw();
		}
		applet.popMatrix();
	}
	
	public void setWallTexture(PImage wallTexture) {
		this.wallTexture = wallTexture;
	}
}