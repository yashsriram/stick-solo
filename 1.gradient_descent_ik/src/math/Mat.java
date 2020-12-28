package math;

import org.ejml.data.FMatrixRMaj;
import org.ejml.dense.row.CommonOps_FDRM;

public class Mat extends FMatrixRMaj {
    /* Constructors */
    protected Mat(int numRows, int numCols) {
        super(numRows, numCols);
    }

    protected Mat(float[] values) {
        super(values);
    }

    public Mat(float[][] values) {
        super(values);
    }

    public Mat(Mat mat) {
        super(mat);
    }

    /* New allocation operations */
    public float norm() {
        int size = this.getNumElements();
        double sumOfSquares = 0.0;
        for (int i = 0; i < size; ++i) {
            sumOfSquares += (data[i] * data[i]);
        }
        return (float) Math.sqrt(sumOfSquares);
    }

    public Mat mult(Mat b) {
        Mat product = new Mat(this.numRows, b.numCols);
        CommonOps_FDRM.mult(this, b, product);
        return product;
    }

    public Vec mult(Vec b) {
        Vec product = new Vec(this.numRows);
        CommonOps_FDRM.mult(this, b, product);
        return product;
    }

    public Mat transpose() {
        Mat transpose = new Mat(this.numCols, this.numRows);
        CommonOps_FDRM.transpose(this, transpose);
        return transpose;
    }

    public Mat inverse() {
        Mat inverted = new Mat(this.numRows, this.numCols);
        boolean isSuccess = CommonOps_FDRM.invert(this, inverted);
        if (!isSuccess) {
            throw new IllegalArgumentException("Can't invert matrix");
        }
        return inverted;
    }

    public float determinant() {
        return CommonOps_FDRM.det(this);
    }

    /* In place operations */
    public Mat plusInPlace(Mat b) {
        CommonOps_FDRM.addEquals(this, b);
        return this;
    }

    public Mat scaleInPlace(float b) {
        CommonOps_FDRM.scale(b, this);
        return this;
    }

    public Mat normalizeInPlace() {
        float norm = norm();
        int size = this.getNumElements();
        for (int i = 0; i < size; ++i) {
            data[i] = data[i] / norm;
        }
        return this;
    }
}
