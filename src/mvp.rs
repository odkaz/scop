extern crate nalgebra_glm as glm;

use crate::matrix::{Matrix, TMatrix4};
use crate::vector::{Vector, TVector3, TVector4};
use crate::camera::Camera;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn view_matrix(r: TVector3<f32>, u: TVector3<f32>, d: TVector3<f32>, p: TVector3<f32>) -> TMatrix4<f32> {
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

pub fn look_at(position: &TVector3<f32>, target: &TVector3<f32>, up: &TVector3<f32>) -> TMatrix4<f32> {

    let cameraDirection = (position - target).normalize();
    let cameraRight = Vector::cross_product(&up, &cameraDirection).normalize();
    let cameraUp = Vector::cross_product(&cameraDirection, &cameraRight);
    view_matrix(cameraRight, cameraUp, cameraDirection, target.clone())
}

pub fn get_mvp(camera: &mut Camera<f32>) -> TMatrix4<f32>{
    let start = SystemTime::now();
    let since_the_epoch = (start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() / 30 % 360) as f32;
    let p = camera.get_pos();
    let mut trans = Matrix::translation(p[0], p[1], p[2]);
    let mut scale = Matrix::scale(0.1, 0.1, 0.1);
    let mut rot = Matrix::rotation(0., since_the_epoch as f32, 0.);
    let mut view = look_at(
        &Vector::from([0., 0., 5.]),
        &Vector::from([0., 0., 0.]),
        &Vector::from([0., 1., 0.]),
    );
    return trans * scale
}
