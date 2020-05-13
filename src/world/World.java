package world;
import ddf.minim.AudioPlayer;
import ddf.minim.Minim;
import math.Vec;
import processing.core.PApplet;
import processing.core.PImage;
import processing.core.PShape;
import robot.sensing.CircleObstacle;
import robot.sensing.Obstacle;

import java.util.ArrayList;
import java.util.List;

public class World {
	public static int SKY_COLOR = 0;//9687551;
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
	private PImage waterTexture;
	private Waterfall waterfall;
	private AudioPlayer rocksAudio;
	private AudioPlayer wind;
	private Minim minim;
	private AudioPlayer waterAudio;
	
	public World(PApplet applet, Vec MIN_CORNER, Vec MAX_CORNER, List<Obstacle> obstacles) {
		this.applet = applet;
		this.MAX_CORNER = MAX_CORNER;
		this.MIN_CORNER = MIN_CORNER;
		
		this.canyonShape = applet.loadShape("mountain_low.obj");
		this.canyonTexture = applet.loadImage("wall_texture.jpg");
		applet.noFill(); 
		applet.noStroke();
		this.canyonShape.setTexture(this.canyonTexture);
		this.canyonShape.scale(40.0f);
		this.landTexture = applet.loadImage("grass.png");
		
		this.obstacleShape = applet.loadShape("rock1.OBJ");
		this.obstacles = obstacles;
		
		this.minim = new Minim(applet);
		this.waterTexture = applet.loadImage("water.png");
		this.waterfall = new Waterfall(applet, new Vec(200, -40));
		this.waterAudio = minim.loadFile("sounds/water.mp3");
		this.waterAudio.play(1000);
		
		this.rocksAudio = minim.loadFile("sounds/rock-debris-fall.mp3");
		this.stoneTexture = applet.loadImage("stone_texture.jpg");
//		loadStoneTexture();

		this.wind = minim.loadFile("sounds/wind01.mp3");
		this.wind.loop();
	}
	
	public void update(float dt) {
        for (Stone stone : stones) {
        	stone.update(dt);
        }
        
        this.waterfall.update(dt);
    }
	
	public void draw() {
    	drawFloor();
    	drawObstacles();
    	drawStones();
    	drawCanyon();
//    	drawTracer();
    	drawWaterfall();
    }
	
	public void spawnStones(Vec position) {
        stones.add(new Stone(position, applet));
        rocksAudio.play(1);
        return;
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
		applet.rotateZ(-PApplet.PI/2);
		applet.translate(820,20,0);
		applet.shape(this.canyonShape);
		applet.popMatrix();
		
		applet.pushMatrix();
		applet.rotateY(PApplet.PI/2);
		applet.rotateZ(PApplet.PI);
		applet.translate(1120, -140, 0);
		applet.shape(this.canyonShape);
		applet.translate(-600, 0, 0);
		applet.shape(this.canyonShape);
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
    
	private void loadStoneTexture() {
		applet.fill(255,255,255,255);
		applet.noStroke();
		applet.noFill();
		stoneShape = applet.createShape(PApplet.SPHERE, 3);
		stoneShape.setTexture(this.stoneTexture);
	}
	
	private void drawWaterfall() {
		this.waterfall.draw(waterTexture);
	}
	
	public void drawObstacles() {
		for(Obstacle obstacle : this.obstacles) {
			if(obstacle instanceof CircleObstacle) {
				Vec position = ((CircleObstacle) obstacle).center;
				applet.pushMatrix();
				applet.shapeMode(PApplet.CENTER);
				applet.translate(-35, position.get(1)+32, position.get(0)-17);
				applet.rotateZ(PApplet.PI/2);
				applet.scale(5.25f);
				applet.shape(this.obstacleShape);
				applet.shapeMode(PApplet.CORNER);
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