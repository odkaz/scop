use crate::matrix::{Matrix, TMatrix4};
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn projection(width: f32, height: f32, degree: f32) -> TMatrix4<f32> {
    let znear = 0.1;
    let zfar = 100.0;
    let fov = degree / 360. * PI;
    let aspect = height / width;
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

pub fn timer() -> f32 {
    let start = SystemTime::now();
    let since_the_epoch = (start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        / 30
        % 360) as f32;
    since_the_epoch
}
