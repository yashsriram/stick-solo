import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;

public class Drawing {
	private PImage wallTexture;
	private PApplet parent;
	private Vec MAX_CORNER;
	private Vec MIN_CORNER;
	private PImage landTexture;
	
	
	public void drawWorld() {
    	drawWall();
    	drawFloor();
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
	public Drawing(PApplet parent, Vec MIN_CORNER, Vec MAX_CORNER) {
		this.parent = parent;
		this.MAX_CORNER = MAX_CORNER;
		this.MIN_CORNER = MIN_CORNER;
		this.wallTexture = parent.loadImage("wall_texture.jpg");
		this.landTexture = parent.loadImage("grass.png");
	}
	public void setWallTexture(PImage wallTexture) {
		this.wallTexture = wallTexture;
	}
}