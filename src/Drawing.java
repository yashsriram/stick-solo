import java.util.ArrayList;
import java.util.List;

import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;
import processing.core.PShape;
import robot.sensing.CircleObstacle;
import robot.sensing.Obstacle;

public class Drawing {
	public static int SKY_COLOR = 9687551;
	private PImage wallTexture;
	private PApplet parent;
	private Vec MAX_CORNER;
	private Vec MIN_CORNER;
	private PImage landTexture;
	private PShape obstacleShape;
	private PImage canyonTexture;
	private PShape canyonShape;
	private List<Obstacle> obstacles;
	public List<Stone> stones = new ArrayList<>();
	private PImage stoneTexture;
	
	
	public void drawWorld() {
//    	drawWall();
    	drawFloor();
    	drawObstacles();
    	drawStones();
    	drawCanyon();
    }
	
	private void drawCanyon() {
		parent.pushMatrix();
		parent.scale(40);
		parent.rotateZ(-PApplet.PI/2);
		parent.translate(20.5f,0.5f,0);
		parent.noFill();
		parent.noStroke();
		this.canyonShape.setTexture(this.canyonTexture);
		parent.shape(this.canyonShape);
		parent.popMatrix();
		
		parent.pushMatrix();
		parent.rotateY(PApplet.PI/2);
		parent.rotateZ(PApplet.PI);
		parent.scale(40);
		parent.translate(28, -3.5f, 0);
		parent.noFill();
		parent.noStroke();
		parent.shape(this.canyonShape);
		parent.translate(-15, 0, 0);
		parent.shape(this.canyonShape);
		parent.popMatrix();
		
	}
    
    private void drawWall() {
    	parent.pushMatrix();
    	parent.noStroke();
    	parent.rotateY(-PApplet.PI/2);
    	parent.beginShape();
    	parent.textureMode(PApplet.NORMAL);
    	parent.texture(this.wallTexture);
    	parent.vertex(MAX_CORNER.get(0), MIN_CORNER.get(1), 0, 0);
    	parent.vertex(MIN_CORNER.get(0), MIN_CORNER.get(1), 1, 0);
    	parent.vertex(MIN_CORNER.get(0), MAX_CORNER.get(1), 1, 1);
    	parent.vertex(MAX_CORNER.get(0), MAX_CORNER.get(1), 0, 1);
    	parent.endShape();
    	parent.popMatrix();
    	
    }
    
    private void drawFloor() {
    	parent.pushMatrix();
    	parent.rotateX(PApplet.PI/2);
    	parent.translate(0, 0, -150);
    	float startX = -500, startY = -500;
    	float sidelen = 200;
    	parent.noStroke();
    	parent.beginShape(PApplet.QUADS);
    	parent.textureMode(PApplet.NORMAL);
    	parent.texture(this.landTexture);
    	for(int i=0; i<5; i++) {
    		for(int j=0; j<5; j++) {
    	    	parent.vertex(startX+i*sidelen, startY+j*sidelen, 0, 0);
    	    	parent.vertex(startX+(i-1)*sidelen, startY+j*sidelen, 1, 0);
    	    	parent.vertex(startX+(i-1)*sidelen, startY+(j+1)*sidelen, 1, 1);
    	    	parent.vertex(startX+i*sidelen, startY+(j+1)*sidelen, 0, 1);
    	    	
    		}
    	}
    	parent.endShape();
    	parent.popMatrix();
    }
	public Drawing(PApplet parent, Vec MIN_CORNER, Vec MAX_CORNER, List<Obstacle> obstacles) {
		this.parent = parent;
		this.MAX_CORNER = MAX_CORNER;
		this.MIN_CORNER = MIN_CORNER;
		this.wallTexture = parent.loadImage("wall_texture.jpg");
		this.stoneTexture = parent.loadImage("stone_texture.jpg");
		this.landTexture = parent.loadImage("grass.png");
		this.obstacleShape = parent.loadShape("rock1.OBJ");
		this.canyonShape = parent.loadShape("mountain.obj");
		this.canyonTexture = parent.loadImage("wall_texture.jpg");
		this.obstacles = obstacles;
	}
	public void drawObstacles() {
		for(Obstacle obstacle : this.obstacles) {
			if(obstacle instanceof CircleObstacle) {
				Vec position = ((CircleObstacle) obstacle).center;
				parent.pushMatrix();
				parent.shapeMode(PApplet.CENTER);
				parent.translate(-35, position.get(1)+32, position.get(0)-17);
				parent.noStroke();
				parent.noFill();
				this.obstacleShape.setTexture(this.wallTexture);
				parent.rotateZ(PApplet.PI/2);
				parent.scale(5.25f);
				parent.shape(this.obstacleShape);
				parent.popMatrix();
			}
		}
	}
	
	private void drawStones() {
		for (Stone stone: stones) {
			parent.pushMatrix();
			parent.rotateY(PApplet.PI/2);
			parent.translate(-stone.position.get(0), stone.position.get(1));
			parent.noStroke();
			parent.noFill();
			PShape rock = parent.createShape(PApplet.SPHERE, 3);
			rock.setTexture(this.stoneTexture);
			parent.shape(rock);
			parent.popMatrix();
		}
	}
	
	public void setWallTexture(PImage wallTexture) {
		this.wallTexture = wallTexture;
	}
}