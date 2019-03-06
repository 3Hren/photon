use std::ops::{Add, AddAssign, Mul};

use crate::{vec3::Vec3, vec4::Vec4};

///
/// Index notation is: i, j - row, column.
#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Matrix4x4<T>([Vec4<T>; 4]);

impl<T: Copy> Matrix4x4<T> {
    pub fn new(v: [[T; 4]; 4]) -> Self {
        Matrix4x4([Vec4::from(v[0]), Vec4::from(v[1]), Vec4::from(v[2]), Vec4::from(v[3])])
    }
}

impl Matrix4x4<f64> {
    pub fn identity() -> Self {
        Matrix4x4::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]])
    }
}

impl Matrix4x4<f64> {
    pub fn inverse(&self) -> Self {
        let s0 = self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1];
        let s1 = self.0[0][0] * self.0[1][2] - self.0[1][0] * self.0[0][2];
        let s2 = self.0[0][0] * self.0[1][3] - self.0[1][0] * self.0[0][3];
        let s3 = self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2];
        let s4 = self.0[0][1] * self.0[1][3] - self.0[1][1] * self.0[0][3];
        let s5 = self.0[0][2] * self.0[1][3] - self.0[1][2] * self.0[0][3];

        let c5 = self.0[2][2] * self.0[3][3] - self.0[3][2] * self.0[2][3];
        let c4 = self.0[2][1] * self.0[3][3] - self.0[3][1] * self.0[2][3];
        let c3 = self.0[2][1] * self.0[3][2] - self.0[3][1] * self.0[2][2];
        let c2 = self.0[2][0] * self.0[3][3] - self.0[3][0] * self.0[2][3];
        let c1 = self.0[2][0] * self.0[3][2] - self.0[3][0] * self.0[2][2];
        let c0 = self.0[2][0] * self.0[3][1] - self.0[3][0] * self.0[2][1];

        let inv_det = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let mut m = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ];

        let a = self.0;

        m[0][0] = ( a[1][1] * c5 - a[1][2] * c4 + a[1][3] * c3) * inv_det;
        m[0][1] = (-a[0][1] * c5 + a[0][2] * c4 - a[0][3] * c3) * inv_det;
        m[0][2] = ( a[3][1] * s5 - a[3][2] * s4 + a[3][3] * s3) * inv_det;
        m[0][3] = (-a[2][1] * s5 + a[2][2] * s4 - a[2][3] * s3) * inv_det;

        m[1][0] = (-a[1][0] * c5 + a[1][2] * c2 - a[1][3] * c1) * inv_det;
        m[1][1] = ( a[0][0] * c5 - a[0][2] * c2 + a[0][3] * c1) * inv_det;
        m[1][2] = (-a[3][0] * s5 + a[3][2] * s2 - a[3][3] * s1) * inv_det;
        m[1][3] = ( a[2][0] * s5 - a[2][2] * s2 + a[2][3] * s1) * inv_det;

        m[2][0] = ( a[1][0] * c4 - a[1][1] * c2 + a[1][3] * c0) * inv_det;
        m[2][1] = (-a[0][0] * c4 + a[0][1] * c2 - a[0][3] * c0) * inv_det;
        m[2][2] = ( a[3][0] * s4 - a[3][1] * s2 + a[3][3] * s0) * inv_det;
        m[2][3] = (-a[2][0] * s4 + a[2][1] * s2 - a[2][3] * s0) * inv_det;

        m[3][0] = (-a[1][0] * c3 + a[1][1] * c1 - a[1][2] * c0) * inv_det;
        m[3][1] = ( a[0][0] * c3 - a[0][1] * c1 + a[0][2] * c0) * inv_det;
        m[3][2] = (-a[3][0] * s3 + a[3][1] * s1 - a[3][2] * s0) * inv_det;
        m[3][3] = ( a[2][0] * s3 - a[2][1] * s1 + a[2][2] * s0) * inv_det;

        Matrix4x4::new(m)
    }

    pub fn transform_normal(n: &Vec3<f64>, inv: Matrix4x4<f64>) -> Vec3<f64> {
        Vec3 {
            x: inv.0[0][0] * n.x + inv.0[1][0] * n.y + inv.0[2][0] * n.z,
            y: inv.0[0][1] * n.x + inv.0[1][1] * n.y + inv.0[2][1] * n.z,
            z: inv.0[0][2] * n.x + inv.0[1][2] * n.y + inv.0[2][2] * n.z,
        }
    }
}

impl<'a, T: Copy + Add<Output = T> + Mul<Output = T>> Mul<Vec4<T>> for &'a Matrix4x4<T> {
    type Output = Vec4<T>;

    fn mul(self, vec: Vec4<T>) -> Self::Output {
        Vec4::new(
            vec[0] * self.0[0][0] + vec[1] * self.0[0][1] + vec[2] * self.0[0][2] + vec[3] * self.0[0][3],
            vec[1] * self.0[1][0] + vec[1] * self.0[1][1] + vec[2] * self.0[1][2] + vec[3] * self.0[1][3],
            vec[2] * self.0[2][0] + vec[1] * self.0[2][1] + vec[2] * self.0[2][2] + vec[3] * self.0[2][3],
            vec[3] * self.0[3][0] + vec[1] * self.0[3][1] + vec[2] * self.0[3][2] + vec[3] * self.0[3][3],
        )
    }
}

impl Mul<Matrix4x4<f64>> for Matrix4x4<f64> {
    type Output = Matrix4x4<f64>;

    fn mul(self, o: Matrix4x4<f64>) -> Self::Output {
        let mut out = Matrix4x4::<f64>::default();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    let mut v = out.0[i][j] + self.0[i][k] * o.0[k][j];
                    out.0[i][j] = v;
                }
            }
        }

        out
    }
}

#[test]
fn mul_matrix_vec() {
    let matrix = &Matrix4x4::new([[1, 0, 0, 10], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
    let vec = Vec4::new(10, 10, 10, 1);

    assert_eq!(Vec4::new(20, 10, 10, 1), matrix * vec);
}

#[test]
fn inverse_identity() {
    let i = Matrix4x4::identity();
    assert_eq!(i, i.inverse());
}
