// #![feature(iterator_try_fold)]
// #![allow()]


use std;
use f32::consts::*;


pub enum Axis
{
    x,
    y,
    z
}

pub fn normalize(arr: &mut [f32; 3])
{
    let len = arr[0] * arr[0] + arr[1] * arr[1] + arr[2] * arr[2];
    let len = len.sqrt();
    for i in 0..3
    {
        arr[i] = arr[i]/len;
    }

}

pub fn rotate_vec(rotate: [[f32;3];3], vec: [f32;3]) -> [f32;3]
{
    let mut result = [0.0f32;3];
        for i in 0..3
        {
            //for j = 0..1
            for k in 0..3
            {
                result[i] += rotate[i][k] * vec[k];
            }
        }
        return result;
}


pub fn dot_product(v1: [f32;3], v2: [f32;3]) -> f32
{
    v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum()
}

pub fn cross_product(v1: [f32;3], v2: [f32;3]) -> [f32; 3]
{
    [v1[1] * v2[2] - v1[2] * v2[1], v1[2] * v2[0] - v1[0] * v2[2], v1[0] * v2[1] - v1[1] * v2[0]]
}

pub fn multiply_3d_matrices(m1: [[f32; 3]; 3], m2: [[f32; 3]; 3]) -> [[f32; 3]; 3]
{
    let mut result = [[0.0f32;3];3];
    for i in 0..3
    {
        for j in 0..3
        {
            for k in 0..3
            {
                result[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }
    return result;
}

pub fn rotate_axis_matrix(axis: [f32;3], rad: f32) ->[[f32; 3]; 3]
{
    let x = axis[0];
    let y = axis[1];
    let z = axis[2];
    let sin = rad.sin();
    let cos = rad.cos();
    let sin_m1 = 1.0 - sin;
    let cos_m1 = 1.0 - cos;
    
    [
        [
            cos + x * x * cos_m1,
            x * y * cos_m1 - z * sin,
            x * z * cos_m1 + y * sin 
        ],
        [
            y * x * cos_m1 + z*sin,
            cos + y * y * cos_m1,
            y * z * cos_m1 - x * sin
        ],
        [
            z * x * cos_m1 - y * sin,
            z * y * cos_m1 + x * sin,
            cos + z * z * cos_m1
        ]
    ]
}

pub fn rotate_z_matrix(rad: f32) -> [[f32; 3]; 3]
{
    [
        [rad.cos(), -(rad.sin()), 0.0],
        [rad.sin(), rad.cos(), 0.0],
        [0.0, 0.0, 1.0]
    ]
}

pub fn rotate_y_matrix(rad: f32) -> [[f32; 3]; 3]
{
    [
        [rad.cos(), 0.0 , rad.sin()],
        [0.0, 1.0, 0.0],
        [-(rad.sin()), 0.0, rad.cos()]
    ]
}

pub fn rotate_x_matrix(rad: f32) -> [[f32; 3]; 3]
{
    [
        [1.0, 0.0, 0.0],
        [0.0, rad.cos(), -(rad.sin())],
        [0.0, rad.sin(), rad.cos()]
    ]
}

pub fn test_rotates()
{
    let rot_x_hard = rotate_x_matrix(FRAC_PI_3);
    let rot_x_calc = rotate_axis_matrix([1.0, 0.0, 0.0], FRAC_PI_3);
    println!("{:?}", rot_x_hard);
    println!("{:?}", rot_x_calc);

    let rot_y_hard = rotate_y_matrix(FRAC_PI_3);
    let rot_y_calc = rotate_axis_matrix([0.0, 1.0, 0.0], FRAC_PI_3);
    println!("{:?}", rot_y_hard);
    println!("{:?}", rot_y_calc);

    let rot_z_hard = rotate_z_matrix(FRAC_PI_3);
    let rot_z_calc = rotate_axis_matrix([0.0, 0.0, 1.0], FRAC_PI_3);
    println!("{:?}", rot_z_hard);
    println!("{:?}", rot_z_calc);


}
