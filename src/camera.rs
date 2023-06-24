extern crate num;

use crate::vector::{Vector, TVector3};
use crate::matrix::Matrix;
use num::{Float, Zero};

#[derive(Debug, Clone)]
pub struct Camera<T> {
    pos: TVector3<T>,
    target: TVector3<T>,
    dir: TVector3<T>,
    right: TVector3<T>,
    up: TVector3<T>
}

impl Camera<f32> {
    pub fn new(position: TVector3<f32>, target: TVector3<f32>, up: TVector3<f32>) -> Camera<f32> {
        let direction = (target.clone() - position.clone()).normalize();
        Camera {
            pos: position.clone(),
            target: target.clone(),
            dir: direction.clone(),
            right: Vector::cross_product(&up, &direction).normalize(),
            up: up,
        }
    }
}

impl Camera<f32> {
    pub fn move_forward(&mut self, scale: f32) {
        let mut buf = self.dir.clone();
        buf.scl(scale);
        println!("f{}", buf);
        self.pos = self.pos.clone() + buf;
    }
    pub fn move_right(&mut self, scale: f32) {
        let mut buf = self.right.clone();
        buf.scl(scale);
        println!("r{}", buf);
        self.pos = self.pos.clone() + buf;
    }
}

impl Camera<f32> {
    pub fn get_pos(&self) -> &TVector3<f32> {
        &self.pos
    }
    pub fn get_target(&self) -> &TVector3<f32> {
        &self.target
    }
    pub fn get_up(&self) -> &TVector3<f32> {
        &self.up
    }
    pub fn get_dir(&self) -> &TVector3<f32> {
        &self.dir
    }
    pub fn get_right(&self) -> &TVector3<f32> {
        &self.right
    }
}

impl Camera<f32> {
    fn view_matrix(r: TVector3<f32>, u: TVector3<f32>, d: TVector3<f32>, p: TVector3<f32>) -> Matrix<f32> {
        let lhs = Matrix::from([
            [r[0], r[1], r[2], 0.],
            [u[0], u[1], u[2], 0.],
            [d[0], d[1], d[2], 0.],
            [0., 0., 0., 1.],
        ]);
        let rhs = Matrix::from([
            [1., 0., 0., -p[0]],
            [0., 1., 0., -p[1]],
            [0., 0., 1., -p[2]],
            [0., 0., 0., 1.],
        ]);
        lhs * rhs
    }

    pub fn look_at(&mut self) -> Matrix<f32> {
        // self.dir = (self.pos.clone() - self.dir.clone()).normalize();
        self.right = Vector::cross_product(&self.up, self.get_dir()).normalize();
        self.up = Vector::cross_product(self.get_dir(), self.get_right()).normalize();
        Camera::view_matrix(self.right.clone(), self.up.clone(), self.dir.clone(), self.pos.clone())
    }
}