package math;

import org.ejml.dense.row.CommonOps_FDRM;

public class Vec extends Mat {
    /* Constructors */
    protected Vec(int numElements) {
        super(numElements, 1);
    }

    public Vec(float... values) {
        super(values);
    }

    public Vec(Vec vec) {
        super(vec);
    }

    /* New allocation operations */
    public void headSet(float... args) {
        if (this.getNumElements() < args.length) {
            throw new IllegalArgumentException("Too many args to set");
        }
        System.arraycopy(args, 0, this.data, 0, args.length);
    }

    public void headSet(Vec b) {
        if (this.getNumElements() < b.getNumElements()) {
            throw new IllegalArgumentException("Too big vector to set");
        }
        for (int i = 0; i < b.getNumElements(); i++) {
            this.set(i, b.get(i));
        }
    }

    public Vec plus(Vec b) {
        Vec sum = new Vec(getNumElements());
        CommonOps_FDRM.add(this, b, sum);
        return sum;
    }

    public Vec minus(Vec b) {
        Vec difference = new Vec(getNumElements());
        CommonOps_FDRM.subtract(this, b, difference);
        return difference;
    }

    public Vec scale(float b) {
        Vec scaled = new Vec(getNumElements());
        CommonOps_FDRM.scale(b, this, scaled);
        return scaled;
    }

    public float dot(Vec b) {
        return CommonOps_FDRM.dot(this, b);
    }

    public static float dist(Vec a, Vec b) {
        assert (a.getNumElements() == b.getNumElements());
        float squaredSum = 0;
        for (int i = 0; i < a.getNumElements(); i++) {
            squaredSum += Math.pow(a.get(i) - b.get(i), 2);
        }
        return (float) Math.sqrt(squaredSum);
    }

    /* In place operations */
    public Vec plusInPlace(Vec b) {
        super.plusInPlace(b);
        return this;
    }

    public Vec scaleInPlace(float b) {
        super.scaleInPlace(b);
        return this;
    }

    public Vec normalizeInPlace() {
        super.normalizeInPlace();
        return this;
    }
}
