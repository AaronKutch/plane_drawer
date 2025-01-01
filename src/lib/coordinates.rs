use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::fmt;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

use fixed::traits::{FromFixed, LossyInto};
use fixed::types::I32F32;
use fixed::types::I52F12;
use fixed::types::I36F28;
use fixed_trigonometry::*;

// use common::*;
use crate::shape::*;

impl Add for D1 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {val: self.val + other.val}
    }
}
impl Sub for D1 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {val: self.val - other.val}
    }
}
impl Mul for D1 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {val: self.val * other.val}
    }
}
impl Div for D1 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {val: self.val / other.val}
    }
}
impl Rem for D1 {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self {val: self.val % other.val}
    }
}
//------------------------------------
impl AddAssign for D1 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl SubAssign for D1 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl MulAssign for D1 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl DivAssign for D1 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
impl RemAssign for D1 {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}
//------------------------------------
impl Neg for D1 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {val: -self.val}
    }
}
impl Ord for D1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
//------------------------------------


//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
impl Add for Angle {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let result = Self {val: (self.val + other.val)};
        result
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let result = Self {val: self.val - other.val};
        result
    }
}impl Div for Angle {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let result = Self {val: self.val / other.val};
        result
    }
}
impl Mul for Angle {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let result = Self {val: self.val * other.val};
        result
    }
}
impl Rem for Angle {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self {val: self.val % other.val}
    }
}
impl AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl MulAssign for Angle {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl DivAssign for Angle {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
impl RemAssign for Angle {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}
impl Neg for Angle {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let result = Self {val: -self.val};
        result
    }
}
impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA




//+++++++++++++++++++++++++++
impl Add for D3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            coords: self.coords + other.coords,
            angle: self.angle + other.angle
        }
    }
}
//------------------------------------
impl Sub for D3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            coords: self.coords - other.coords,
            angle: self.angle - other.angle
        }
    }
}
//+++++++++++++++++++++++++++
impl AddAssign for D3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        self.coords += other.coords;
        self.angle += other.angle;
    }
}
//------------------------------------
impl SubAssign for D3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl SubAssign for Pos {
    fn sub_assign(&mut self, other: Self) {
        self.coords -= other.coords;
        self.angle -= other.angle;
    }
}
//####################################
impl MulAssign for D3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
////////////////////////////////////
impl DivAssign for D3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
//------------------------------------
impl Neg for D3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}
//####################################
impl Mul for D3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}
////////////////////////////////////
impl Div for D3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl fmt::Debug for D1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}", self.val)
    }
}
impl<T: fmt::Debug> fmt::Debug for D2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:?}, y: {:?}", self.x, self.y)
    }
}
impl fmt::Debug for D3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x.val, self.y.val, self.z.val)
    }
}
impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}, a: {:?}", self.coords.x.val, self.coords.y.val, self.coords.z.val, self.angle.val)
    }
}
impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a: {}", self.val)
    }
}


#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub coords: D3,
    pub angle: Angle,
}
impl Pos {
    pub fn new(coords: D3, angle: Angle) -> Pos {
        Pos {
            coords,
            angle,
        }
    }
    pub fn zero() -> Pos {
        Pos {
            coords: D3::zero(),
            angle: Angle::zero(),
        }
    }
    pub fn xy(&self) -> D2<D1> {
        self.coords.xy()
    }
    pub fn x(&self) -> D1 {
        self.coords.x
    }
    pub fn y(&self) -> D1 {
        self.coords.y
    }
    pub fn z(&self) -> D1 {
        self.coords.z
    }
    pub fn transform(&self, from: Pos, to: Pos) -> Self {
        let coords = self.coords.transform(from, to);
        let angle = self.angle.transform(from, to);
        return Pos::from((coords, angle))
    }
    pub fn rotate(&self, angle: Angle) -> Pos {
        let coords = self.coords.rotate(angle);
        let angle = self.angle + angle;
        return Pos::from((coords, angle))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct D3 {
    pub x: D1,
    pub y: D1,
    pub z: D1,
}
impl D3 {
    pub fn zero() -> D3 {
        D3 {
            x: D1::from(0),
            y: D1::from(0),
            z: D1::from(0),
        }
    }
    pub fn two() -> D3 {
        D3 {
            x: D1::from(2),
            y: D1::from(2),
            z: D1::from(2),
        }
    }
    pub fn xy(&self) -> D2<D1> {
        D2::from((self.x, self.y))
    }
    pub fn xyz(&self) -> (i32, i32, i32) {
        (self.x.to_i32(), self.y.to_i32(), self.z.to_i32())
    }
    pub fn to_f32(self) -> (f32, f32, f32) {
        (self.x.to_f32(), self.y.to_f32(), self.z.to_f32())
    }
    /// drops z
    pub fn to_d2(&self) -> D2<D1> {
        D2::from((self.x, self.y))
    }
    pub fn rotate(self, angle: Angle) -> D3 {
        let trig_mult = TrigMult::new(angle);
        return self.rotate_with_trig_mult(trig_mult)
    }
    pub fn rotate_with_trig_mult(self, trig_mult: TrigMult) -> D3 {
        let x = I52F12::from_num(self.x.val) * I52F12::from_num(trig_mult.cos) - I52F12::from_num(self.y.val) * I52F12::from_num(trig_mult.sin);
        let y = I52F12::from_num(self.x.val) * I52F12::from_num(trig_mult.sin) + I52F12::from_num(self.y.val) * I52F12::from_num(trig_mult.cos);
        
        return D3::from((D1::from(I52F12::from_num(x)), D1::from(I52F12::from_num(y)), self.z))
    }
    pub fn transform(self, from: Pos, to: Pos) -> D3 {
        // difference between surface coords
        let dxyz = from.coords - to.coords;
        let center = dxyz.rotate(-to.angle);
        // center of surface 1 from surface 2's reference frame?
    
        let delta_angle = from.angle - to.angle;
        // surface 1's rotation from surface 2's reference frame?
        let yeet = self.rotate(delta_angle);
        // rotates point on surface 1 to surface 2's reference frame?
    
        return yeet + center // add difference in surface position
    }
    pub fn cross(self, other: D3) -> D3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        return D3::from((x, y, z))
    }
    pub fn dot(self, other: D3) -> D1 {
        return self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn magnitude(self) -> D1 {
        return D1::from((self.x.power_pos(2) + self.y.power_pos(2) + self.z.power_pos(2)).sqrt())
    }
    /// sets the magnitude to mag
    pub fn normalize(self, mag: D1) -> D3 {
        let self_mag = self.magnitude();
        if mag.is_zero() {
            return D3::zero();
        }
        return self / D3::from(self_mag) * D3::from(mag)
    }
    pub fn distance(self, other: Self) -> D1 {
        D1::from((self.x - other.x).power_pos(2) + (self.y - other.y).power_pos(2) + (self.z - other.z).power_pos(2)).sqrt()
    }
    pub fn floor(self) -> D3 {
        D3::from((self.x.floor(), self.y.floor(), self.z.floor()))
    }
    pub fn round_to(self, unit: D1) -> D3 {
        D3::from((self.x.round_to(unit), self.y.round_to(unit), self.z.round_to(unit)))
    }
    /// clamps the magnitude between mag_min and mag_max (unless magnitude is 0, then it stays 0)
    /// saturation speed is multiplied with self to increase the speed of saturation
    pub fn saturate(self, mag_min: D1, mag_max: D1, sat_speed: D1) -> D3 {
        let num = self * D3::from(sat_speed);
        if num.magnitude() > mag_max {
            //println!("maxed out D3");
            return num.normalize(mag_max);
        } else if num.magnitude() < mag_min {
            //println!("minned out D3");
            return num.normalize(mag_min);
        } else {
            return num
        }
    }
}

//---------------------------------------------------------
impl<T: Add<Output = T>> Add for D2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for D2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Mul<Output = T>> Mul for D2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<T: Div<Output = T>> Div for D2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}
impl<T: Rem<Output = T>> Rem for D2<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y
        }
    }
}

impl<T: AddAssign> AddAssign for D2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl<T: SubAssign> SubAssign for D2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl<T: MulAssign> MulAssign for D2<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl<T: DivAssign> DivAssign for D2<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}
impl<T: Neg<Output = T>> Neg for D2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}
//---------------------------------------------------------

/// for 2d points
pub trait TrigStuff: Sized + Sub<Output = Self> + Add<Output = Self> + Copy {
    fn rotate_with_trig_mult(&self, trig_mult: TrigMult) -> Self;
    // -----------everything below this is a generic trait fn or something-------------
    fn rotate(self, angle: Angle) -> Self {
        return self.rotate_with_trig_mult(TrigMult::new(angle))
    }
    fn rotate_about(self, about: Self, angle: Angle) -> Self {
        let self_coords = self - about;
        let rotated_coords = self_coords.rotate(angle);
        return rotated_coords + about
    }
    fn transform(self, from: Self, angle: Angle, to: Self, angle_to: Angle) -> Self {
        let dxy = from - to;
        let center = dxy.rotate(-angle_to);
        let delta_angle = angle - angle_to;
        let yeet = self.rotate(delta_angle);
        return yeet + center
    }
    // ---------------------------------------------------------------------------------
}

impl TrigStuff for D2<D1> {
    fn rotate_with_trig_mult(&self, trig_mult: TrigMult) -> Self {
        let x = I52F12::from_num(self.x.val) * I52F12::from_num(trig_mult.cos) - I52F12::from_num(self.y.val) * I52F12::from_num(trig_mult.sin);
        let y = I52F12::from_num(self.x.val) * I52F12::from_num(trig_mult.sin) + I52F12::from_num(self.y.val) * I52F12::from_num(trig_mult.cos);
        // return D2::from((D1::from(I52F12::from_num(x)), D1::from(I52F12::from_num(y))))
        return D2::from((x, y))
    }
    
}

/// for 1d points that can be non-whole numbers
pub trait FractionStuff: BasicMath {
    fn floor(&self) -> Self; // fraction
    fn ceil(&self) -> Self; // fraction
    fn round(&self) -> Self; // fraction
    // ------- generics below this line -------
    fn floor_to(self, unit: Self) -> Self {
        let num = self / unit;
        return num.floor() * unit
    }
    fn ceil_to(self, unit: Self) -> Self {
        let num = self / unit;
        return num.ceil() * unit
    }
    fn round_to(self, unit: Self) -> Self {
        let num = self / unit;
        return num.round() * unit
    }
    // ----------------------------------------
}
impl FractionStuff for D1 {
    fn floor(&self) -> Self {
        D1::from(self.val.floor())
    }
    fn ceil(&self) -> Self {
        D1::from(self.val.ceil())
    }
    fn round(&self) -> Self {
        D1::from(self.val.round())
    }
}

/// for 1d points. If something implements this, most of the important methods besides trigonometry will work
pub trait Stuff: BasicMath {
    fn power(self, power: i32) -> Self; // neg
    fn abs(&self) -> Self; // neg

    fn sqrt(self) -> Self;
    fn power_pos(self, power: usize) -> Self;
    fn is_zero(&self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
}
impl Stuff for D1 {
    fn power_pos(self, power: usize) -> Self {
        Self {val: powi(self.val, power)}
    }
    fn power(self, power: i32) -> Self {
        if power >= 0 {
            return self.power_pos(power as usize)
        }
        let yeet = D1::one() / D1::from(powi(self.val, power.abs() as usize));
        return yeet
    }
    fn sqrt(self) -> Self {
        Self {val: self.val.sqrt()}
    }
    fn is_zero(&self) -> bool {
        return self.val == 0
    }
    fn zero() -> D1 {
        D1 {val: I36F28::from_num(0)}
    }
    fn one() -> D1 {
        D1 {val: I36F28::from_num(1)}
    }
    fn two() -> D1 {
        D1 {val: I36F28::from_num(2)}
    }
    fn abs(&self) -> D1 {
        D1 {val: self.val.abs()}
    }
}

impl Stuff for u16 {
    fn power_pos(self, power: usize) -> Self {
        self.pow(power as u32)
    }
    fn power(self, power: i32) -> Self {
        if power >= 0 {
            return self.power_pos(power as usize)
        }
        let yeet = 1 / self.pow(power.abs() as u32);
        return yeet
    }
    fn sqrt(self) -> Self {
        println!("sqrt not implimented for u16");
        self
    }
    fn is_zero(&self) -> bool {return *self == 0}
    fn abs(&self) -> Self {*self}
    fn zero() -> Self {0}
    fn one() -> Self {1}
    fn two() -> Self {2}
}

/// automatically implemented for all types that implement Add, Sub, Mul, Div, Ord, Copy, From<u8>
pub trait BasicMath: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Ord + Copy + From<u8> {
    fn clamp_min(self, min: Self) -> Self;
    fn clamp_max(self, max: Self) -> Self;
    fn checked_sub(self, other: Self) -> Option<Self>;
}
impl<T: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Ord + Copy + From<u8>> BasicMath for T {
    fn clamp_min(self, min: Self) -> Self {
        if self < min {
            return min;
        } else {
            return self;
        }
    }
    fn clamp_max(self, max: Self) -> Self {
        if self > max {
            return max;
        } else {
            return self;
        }
    }
    fn checked_sub(self, other: Self) -> Option<Self> {
        if self < other {
            return None;
        } else {
            return Some(self - other);
        }
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct D2<T> {
    pub x: T,
    pub y: T,
}
impl D2<D1> {
    pub fn zero() -> D2<D1> {
        D2 {
            x: D1::from(0),
            y: D1::from(0),
        }
    }
    pub fn one() -> D2<D1> {
        D2 {
            x: D1::from(1),
            y: D1::from(1),
        }
    }
    pub fn two() -> D2<D1> {
        D2 {
            x: D1::from(2),
            y: D1::from(2),
        }
    }
    pub fn to_angle(self) -> Angle {
        let tan = atan::atan2(self.y.val, self.x.val);
        Angle::from(I32F32::from_fixed(tan))
    }
    pub fn from_polar(angle: Angle, r: D1) -> D2<D1> {
        let coords = D2::from((
            D1::from(angle.cos() * I32F32::from_fixed(r.val)),
            D1::from(angle.sin() * I32F32::from_fixed(r.val))));
        return coords
    }
    pub fn to_usize(self) -> (usize, usize) {
        (self.x.to_usize(), self.y.to_usize())
    }
    pub fn to_u32(self) -> (u32, u32) {
        (self.x.to_usize() as u32, self.y.to_usize() as u32)
    }
    pub fn to_f32(self) -> (f32, f32) {
        (self.x.to_f32(), self.y.to_f32())
    }
    // must round towards negative infinity
    pub fn to_i32(self) -> (i32, i32) {
        (self.x.to_i32(), self.y.to_i32())
    }
}

impl<T> D2<T> {
    pub const fn new_const(x: T, y: T) -> D2<T> {
        D2 {x,y}
    }
    pub fn swap_xy(self) -> Self {
        D2::from((self.y, self.x))
    }
}

impl<T: BasicMath> D2<T> {
    pub fn cross(self, other: Self) -> T {
        return self.x * other.y - self.y * other.x;
    }
    pub fn dot(self, other: Self) -> T {
        return self.x * other.x + self.y * other.y
    }
    pub fn clamp_min(&self, other: Self) -> Self {
        D2::from((self.x.clamp_min(other.x), self.y.clamp_min(other.y)))
    }
    pub fn clamp_max(&self, other: Self) -> Self {
        D2::from((self.x.clamp_max(other.x), self.y.clamp_max(other.y)))
    }
    pub fn checked_sub(&self, other: Self) -> Option<Self> {
        let x = self.x.checked_sub(other.x)?;
        let y = self.y.checked_sub(other.y)?;
        Some(D2::from((x, y)))
    }
}

impl<T: BasicMath + Neg<Output = T>> D2<T> {
    /// avoids trig functions and rounding error
    pub fn rotate_quarters(self, about: Self, direction: Direction) -> Self {
        let self_coords = self - about;
        let rotated_coords = match direction {
            Direction::N => D2::from((-self_coords.y, self_coords.x)),
            Direction::S => D2::from((self_coords.y, -self_coords.x)),
            Direction::E => self_coords,
            Direction::W => D2::from((-self_coords.x, -self_coords.y)),
        };
        return rotated_coords + about
    }
}

impl<T: FractionStuff> D2<T> {
    pub fn floor(self) -> D2<T> {
        (self.x.floor() , self.y.floor()).into()
    }
    pub fn ceil(self) -> D2<T> {
        (self.x.ceil() , self.y.ceil()).into()
    }
    pub fn round(self) -> D2<T> {
        (self.x.round() , self.y.round()).into()
    }
    pub fn round_to(self, unit: Self) -> Self {
        D2::from((self.x.round_to(unit.x), self.y.round_to(unit.y)))
    }
    pub fn floor_to(self, unit: Self) -> Self {
        D2::from((self.x.floor_to(unit.x), self.y.floor_to(unit.y)))
    }
    pub fn ceil_to(self, unit: Self) -> Self {
        D2::from((self.x.ceil_to(unit.x), self.y.ceil_to(unit.y)))
    }
}

impl<T: Stuff> D2<T> {
    pub fn distance(self, other: Self) -> T {
        //((self.x - other.x).power_pos(2) + (self.y - other.y).power_pos(2)).sqrt()
        ((self.x - other.x)*(self.x - other.x) + (self.y - other.y)*(self.y - other.y)).sqrt()
    }
    pub fn magnitude(self) -> T {
        return (self.x*self.x + self.y*self.y).sqrt()
    }
    /// scales the vector so that the magnitude of the vector is 1
    /// returns D2::(0, 0) if magnitude is 0
    pub fn normalize(self) -> D2<T> {
        let self_mag = self.magnitude();
        if self_mag == T::from(0) {
            return self;
        }
        return self / D2::from(self_mag)
    }
    /// clamps the magnitude under mag_max
    pub fn saturate(self, mag_min: T, mag_max: T, sat_speed: T) -> D2<T> {
        let num = self * D2::from(sat_speed);
        if num.magnitude() > mag_max {
            return num.normalize() * D2::from(mag_max);
        } else if num.magnitude() < mag_min {
            return num.normalize() * D2::from(mag_min);
        } else {
            return num
        }
    }
}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct D1 {
    pub val: I36F28,
}
impl D1 {
    pub const fn new_const(val: &str) -> D1 {
        D1 {
            val: I36F28::lit(val)
        }
    }
    pub fn max() -> D1 {
        D1 {
            val: I36F28::MAX,
        }
    }
    pub fn min() -> D1 {
        D1 {
            val: I36F28::MIN,
        }
    }
    pub fn to_f32(self) -> f32 {
        self.val.lossy_into()
    }
    // must round towards negative infinity
    pub fn to_i32(self) -> i32 {
        let yeet:i64 = self.floor().val.lossy_into();
        yeet as i32
    }
    pub fn to_usize(self) -> usize {
        let yeet:i64 = self.val.lossy_into();
        yeet as usize
    }
    pub fn to_u16(self) -> u16 {
        let yeet:i64 = self.val.lossy_into();
        yeet as u16
    }
    pub fn to_u8(self) -> u8 {
        let yeet:i64 = self.val.lossy_into();
        yeet as u8
    }
    pub fn div_euclid(self, other: D1) -> D1 {
        Self {val: (self.val).div_euclid(other.val)}
    }
    // pub fn normalize(self) -> D1 {
    //     if self.is_zero() {
    //         return D1::zero();
    //     } else if self > D1::zero() {
    //         return D1::one();
    //     } else {
    //         return -D1::one();
    //     }
    // }
    pub fn saturate(self, min: D1, max: D1, sat_speed: D1) -> D1 {
        let num = self * sat_speed;
        if num > max {
            //println!("maxed out D1");
            return max;
        } else if num < min {
            //println!("minned out D1");
            return min;
        } else {
            return num;
        }
    }
    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Angle {
    pub val: I32F32
}
impl Angle {
    pub fn zero() -> Self {
        Angle {
            val: I32F32::from_num(0.0) // maybe use lit() to make it const?
        }
    }
    pub fn pi() -> Self {
        Self::from("3.1415926535897")
    }
    pub fn two_pi() -> Self {
        Self::from("6.2831853071796")
    }
    pub fn half_pi() -> Self {
        Self::from("1.5707963267949")
    }
    pub fn three_halves_pi() -> Self {
        Self::from("4.7123889803847")
    }
    pub fn one_fourth_pi() -> Self {
        Self::from("0.7853981633974")
    }
    pub fn sin(&self) -> I32F32 {
        sin(self.val)
    }
    pub fn cos(&self) -> I32F32 {
        cos(self.val)
    }
    pub fn min() -> Angle {
        Angle::from("-3.14")
    }
    pub fn max() -> Angle {
        Angle::from("3.14")
    }
    pub fn abs(&self) -> Angle {
        Angle::from(self.val.abs())
    }
    pub fn small_step() -> Angle {
        Angle::from("0.1")
    }
    // wraps the angle between -pi and pi
    pub fn wrap(&mut self) {
        //self % Angle::max()
        while *self < Angle::min() {
            *self += Angle::two_pi();
        }
        while *self > Angle::max() {
            *self -= Angle::two_pi();
        }
    }
    pub fn transform(self, from: Pos, to: Pos) -> Self {
        let mut angle = self + from.angle - to.angle;
        angle.wrap();
        return angle
    }
    pub fn is_opposite(self, other: Angle) -> bool {
        let diff = self - other;
        let diff = diff.abs() - Angle::pi();
        println!("diff: {:?}", diff);
        return diff.abs() < Angle::small_step() // hack to account for any possible rounding
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TrigMult {
    pub sin: I32F32,
    pub cos: I32F32
}
impl TrigMult {
    pub fn new(angle: Angle) -> Self {
        TrigMult {
            sin: angle.sin(),
            cos: angle.cos()
        }
    }
}