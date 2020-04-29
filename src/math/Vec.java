package math;

import org.ejml.dense.row.CommonOps_FDRM;

public class Vec extends Mat {
    /* Constructors */
    protected Vec(int numElements) {
        super(numElements, 1);
    }

    public Vec(float[] values) {
        super(values);
    }

    public Vec(Vec vec) {
        super(vec);
    }

    /* Manipulators */
    public void headSet(float... args) {
        System.arraycopy(args, 0, this.data, 0, args.length);
    }

    public Vec plusInPlace(Mat b) {
        super.plusInPlace(b);
        return this;
    }

    public Vec minus(Vec b) {
        Vec sum = new Vec(getNumElements());
        CommonOps_FDRM.subtract(this, b, sum);
        return sum;
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
