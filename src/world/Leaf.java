package world;

import math.Vec;
import processing.core.PApplet;
import processing.core.PConstants;
import processing.core.PImage;

public class Leaf {
    Vec position ;
    Vec velocity ;
    float side ;
    float lifetime ;
    private PApplet pApplet;
    PImage texture ;

    public Leaf(Vec position, Vec velocity, float side, float lifetime, PApplet pApplet) {
        this.position = position;
        this.velocity = velocity;
        this.side = side;
        this.lifetime = lifetime;
        this.pApplet = pApplet;
        this.texture = pApplet.loadImage("leaf1.png");
    }

    public boolean update(float dt, float size, Vec wind){
        this.velocity.plusInPlace(wind.scale(0.01f));
        if(lifetime > 0){
            this.position.plusInPlace(this.velocity.scale(dt));
            this.lifetime -= 1 ;
            return true ;
        }else{
            revive(size);
            return false ;
        }
    }

    public void draw(){
        pApplet.pushMatrix();
        pApplet.translate(0, this.position.get(1), this.position.get(0));
        pApplet.rotateY(pApplet.PI/2);
        pApplet.beginShape();
        pApplet.fill(0);
        pApplet.stroke(0);
        pApplet.textureMode(PConstants.NORMAL);
        pApplet.texture(this.texture);
        pApplet.vertex(0, 0, 0, 0);
        pApplet.vertex(0, side, 1, 0);
        pApplet.vertex(side, side, 1, 1);
        pApplet.vertex(side, 0, 0, 1);
        pApplet.endShape();
        pApplet.popMatrix();
    }

    public void revive(float size){
        lifetime = pApplet.random(500, 1000) ;
        this.position = new Vec(size*pApplet.random(-2, -1f), size*pApplet.random(-1, 0f));
        this.velocity = new Vec(pApplet.random(0, 1), pApplet.random(2, 4));
    }
}
