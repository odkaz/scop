use std::clone::Clone;
use std::default::Default;
use std::f32::consts::PI;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub type TMatrix<T, const M: usize, const N: usize> = Matrix<T, M, N>;
pub type TMatrix2<T> = TMatrix<T, 2, 2>;
pub type TMatrix3<T> = TMatrix<T, 3, 3>;
pub type TMatrix4<T> = TMatrix<T, 4, 4>;

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vec<Vec<T>>,
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(s: [[T; N]; M]) -> Matrix<T, M, N> {
        let mut d = Vec::new();
        for item in s.iter() {
            d.push(item.to_vec());
        }
        return Matrix { data: d };
    }
}

impl<T: std::fmt::Debug, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn out(&self) {
        for item in self.data.iter() {
            println!("{:?}", item);
        }
    }
}

impl<T: Display + Add<Output = T> + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn add(&mut self, v: &Matrix<T, M, N>) {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() + item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T: Display + Sub<Output = T> + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn sub(&mut self, v: &Matrix<T, M, N>) {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() - item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T: Display + Mul<Output = T> + Clone + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn scl(&mut self, a: T) {
        let mut res = Vec::new();
        for j in 0..M {
            let it = self.data[j].iter();
            let mut v = Vec::new();
            for item in it {
                v.push(item.clone() * a);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T, const M: usize, const N: usize, const H: usize> Mul<Matrix<T, N, H>> for Matrix<T, M, N>
where
    T: Default + Add<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Matrix<T, M, H>;
    fn mul(self, rhs: Matrix<T, N, H>) -> Matrix<T, M, H> {
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..H {
                let mut sum: T = Default::default();
                for k in 0..N {
                    sum = sum + (self.data[j][k] * rhs.data[k][i]);
                }
                v.push(sum);
            }
            res.push(v);
        }
        Matrix { data: res }
    }
}

pub fn identity_array() -> [[f32; 4]; 4] {
    let mut trans: [[f32; 4]; 4] = [[0.; 4]; 4];
    trans[0][0] = 1.;
    trans[1][1] = 1.;
    trans[2][2] = 1.;
    trans[3][3] = 1.;
    trans
}

impl TMatrix4<f32> {
    pub fn identity() -> TMatrix4<f32> {
        let trans: [[f32; 4]; 4] = identity_array();
        Matrix::from(trans)
    }

    pub fn translation(x: f32, y: f32, z: f32) -> TMatrix4<f32> {
        let mut trans: [[f32; 4]; 4] = identity_array();
        trans[0][3] = x;
        trans[1][3] = y;
        trans[2][3] = z;
        Matrix::from(trans)
    }

    pub fn scale(x: f32, y: f32, z: f32) -> TMatrix4<f32> {
        let mut scale: [[f32; 4]; 4] = identity_array();
        scale[0][0] = x;
        scale[1][1] = y;
        scale[2][2] = z;
        Matrix::from(scale)
    }
}

impl TMatrix4<f32> {
    pub fn as_mut_arr(&self) -> [[f32; 4]; 4] {
        let iter = self.data.iter();
        let mut res: [[f32; 4]; 4] = [[0.; 4]; 4];
        for (i, row) in iter.enumerate() {
            let slice = row.as_slice();
            res[i] = slice.try_into().unwrap();
        }
        res
    }
}

fn degree_to_radians(degree: f32) -> f32 {
    degree / 180. * PI
}

impl TMatrix4<f32> {
    pub fn rotation_x(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_x: [[f32; 4]; 4] = identity_array();
        rot_x[1][1] = f32::cos(t);
        rot_x[1][2] = -f32::sin(t);
        rot_x[2][1] = f32::sin(t);
        rot_x[2][2] = f32::cos(t);
        Matrix::from(rot_x)
    }

    pub fn rotation_y(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_y: [[f32; 4]; 4] = identity_array();
        rot_y[0][0] = f32::cos(t);
        rot_y[0][2] = f32::sin(t);
        rot_y[2][0] = -f32::sin(t);
        rot_y[2][2] = f32::cos(t);
        Matrix::from(rot_y)
    }

    pub fn rotation_z(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_z: [[f32; 4]; 4] = identity_array();
        rot_z[0][0] = f32::cos(t);
        rot_z[0][1] = -f32::sin(t);
        rot_z[1][0] = f32::sin(t);
        rot_z[1][1] = f32::cos(t);
        Matrix::from(rot_z)
    }

    pub fn rotation(x: f32, y: f32, z: f32) -> TMatrix4<f32> {
        let rot_x = Self::rotation_x(x);
        let rot_y = Self::rotation_y(y);
        let rot_z = Self::rotation_z(z);
        // Self::rotation_x(x) * Self::rotation_y(t_y) * Self::rotation_z(t_z)
        rot_x * rot_y * rot_z
    }
}

impl<T: Display, const M: usize, const N: usize> Display for Matrix<T, M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for j in 0..M {
            if j != 0 {
                write!(f, "\n").unwrap();
            }
            write!(f, "[").unwrap();
            for i in 0..N {
                if i != 0 {
                    write!(f, ", ").unwrap();
                }
                write!(f, "{}", self.data[j][i]).unwrap();
            }
            write!(f, "]").unwrap();
        }

        Ok(())
    }
}
