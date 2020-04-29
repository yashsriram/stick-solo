package math;

import org.ejml.data.FMatrixRMaj;
import org.ejml.dense.row.CommonOps_FDRM;

public class Mat extends FMatrixRMaj {
    /* Constructors */
    protected Mat(int numRows, int numCols) {
        super(numRows, numCols);
    }

    public Mat(float[][] values) {
        super(values);
    }

    public Mat(float[] values) {
        super(values);
    }

    public Mat(Mat mat) {
        super(mat);
    }

    /* Manipulators */
    public Mat plusInPlace(Mat b) {
        CommonOps_FDRM.addEquals(this, b);
        return this;
    }

    public Mat minus(Mat b) {
        Mat sum = new Mat(getNumRows(), getNumCols());
        CommonOps_FDRM.subtract(this, b, sum);
        return sum;
    }

    public Mat scaleInPlace(float b) {
        CommonOps_FDRM.scale(b, this);
        return this;
    }

    public float norm() {
        int size = this.getNumElements();
        double sumOfSquares = 0.0;
        for (int i = 0; i < size; ++i) {
            sumOfSquares += (data[i] * data[i]);
        }
        return (float) Math.sqrt(sumOfSquares);
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
