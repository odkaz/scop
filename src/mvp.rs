use crate::matrix::{Matrix, TMatrix4};
use crate::vector::{Vector, TVector3, TVector4};
use crate::camera::Camera;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn projection() -> TMatrix4<f32> {
    let znear = 0.1;
    let zfar = 100.0;
    let width = 600.;
    let height = 600.;
    let fov = 135. / 360. * PI;
    let aspect = width / height;
    make_perspective(fov, aspect, znear, zfar)
}

pub fn make_perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> TMatrix4<f32> {
    let mut arr = [[0.; 4]; 4];
    arr[0][0] = aspect * (1. / f32::tan(fov / 2.));
    arr[1][1] = 1. / f32::tan(fov / 2.);
    arr[2][2] = zfar / (zfar - znear);
    arr[2][3] = (-zfar * znear) / (zfar - znear);
    arr[3][2] = 1.;
    Matrix::from(arr)
}

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
    view_matrix(cameraRight, cameraUp, cameraDirection, position.clone())
}

pub fn timer() -> f32 {
    let start = SystemTime::now();
    let since_the_epoch = (start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() / 30 % 360) as f32;
    since_the_epoch
}

pub fn get_mvp(camera: &mut Camera) -> TMatrix4<f32>{
    let since_the_epoch = timer();
    let p = camera.get_pos();
    let mut trans = Matrix::translation(0., 0., 0.);
    let mut scale = Matrix::scale(1., 1., 1.);
    let mut rot = Matrix::rotation(0., since_the_epoch as f32, 0.);
    let mut view = camera.look_at();
    let mut per = projection();
    return per * view * trans * rot * scale
}
