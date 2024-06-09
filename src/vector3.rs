use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
#[wasm_bindgen]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_u32(x: u32, y: u32, z: u32) -> Vector3 {
        Vector3 {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            z: z.try_into().unwrap(),
        }
    }

    pub fn from_i32(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            z: z.try_into().unwrap(),
        }
    }
}

impl From<JsValue> for Vector3 {
    fn from(value: JsValue) -> Self {
        from_value(value).unwrap()
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Self::Output {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[allow(dead_code)]
impl Vector3 {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn angle(&self, other: Vector3) -> f32 {
        (self.dot(&other) / (self.magnitude() * other.magnitude())).acos()
    }

    pub fn angle_deg(&self, other: Vector3) -> f32 {
        (self.dot(&other) / (self.magnitude() * other.magnitude())).acos() * (180.0 / PI)
    }

    pub fn normalize(&self) -> Vector3 {
        self.clone() / self.magnitude()
    }
}
