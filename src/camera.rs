use crate::matrix::{Matrix, TMatrix4};
use crate::vector::{TVector3, Vector};

#[derive(Debug, Clone)]
pub struct Camera {
    pos: TVector3<f32>,
    target: TVector3<f32>,
    dir: TVector3<f32>,
    right: TVector3<f32>,
    up: TVector3<f32>,
}

impl Camera {
    pub fn new(position: TVector3<f32>, target: TVector3<f32>, up: TVector3<f32>) -> Camera {
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

impl Camera {
    pub fn move_forward(&mut self, scale: f32) {
        let mut buf = self.dir.clone();
        buf.scl(scale);
        self.pos = self.pos.clone() + buf;
    }
    pub fn move_right(&mut self, scale: f32) {
        let mut buf = self.right.clone();
        buf.scl(scale);
        self.pos = self.pos.clone() + buf;
    }
}

impl Camera {
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

impl Camera {
    fn view_matrix(
        r: TVector3<f32>,
        u: TVector3<f32>,
        d: TVector3<f32>,
        p: TVector3<f32>,
    ) -> TMatrix4<f32> {
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

    pub fn look_at(&mut self) -> TMatrix4<f32> {
        // self.dir = (self.pos.clone() - self.dir.clone()).normalize();
        self.right = Vector::cross_product(&self.up, self.get_dir()).normalize();
        self.up = Vector::cross_product(self.get_dir(), self.get_right()).normalize();
        Camera::view_matrix(
            self.right.clone(),
            self.up.clone(),
            self.dir.clone(),
            self.pos.clone(),
        )
    }
}
