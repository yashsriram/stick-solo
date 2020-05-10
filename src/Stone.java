import math.Vec;

public class Stone {
	Vec position;
	Vec velocity;
	Vec acceleration;
	
	public void update(float dt) {
		if(position.get(1) < 100) {
			velocity.plusInPlace(acceleration.scale(dt));
			position.plusInPlace(velocity.scale(dt));
		}
	}
	
	public Stone(Vec position) {
		this.position = position;
		this.velocity = new Vec(0,0);
		this.acceleration = new Vec(0,10);
	}

}
