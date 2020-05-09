import java.util.ArrayList;
import java.util.List;

import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;
import processing.core.PShape;
import robot.sensing.CircleObstacle;
import robot.sensing.Obstacle;

public class Drawing {
	private PImage wallTexture;
	private PApplet parent;
	private Vec MAX_CORNER;
	private Vec MIN_CORNER;
	private PImage landTexture;
	private PShape obstacleShape;
	private List<Obstacle> obstacles;
	public List<Stone> stones = new ArrayList<>();
	private PImage stoneTexture;
	
	
	public void drawWorld() {
    	drawWall();
    	drawFloor();
    	drawObstacles();
    	drawStones();
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
    	parent.translate(-50, 0, MIN_CORNER.get(0));
    	parent.beginShape();
    	parent.textureMode(PApplet.NORMAL);
    	parent.texture(this.landTexture);
    	parent.vertex(100, -100, 0, 0);
    	parent.vertex(-100, -100, 1, 0);
    	parent.vertex(-100, 100, 1, 1);
    	parent.vertex(100, 100, 0, 1);
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