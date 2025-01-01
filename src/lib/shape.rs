
use serde::{Serialize, Deserialize};
use std::ops::Neg;
// use common::*;
use crate::coordinates::*;



#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    N,
    E,
    S,
    W,
}
impl Direction {
    pub fn rotate(&mut self, clockwise: bool) {
        match self {
            Direction::N => {
                if clockwise {
                    *self = Direction::E;
                } else {
                    *self = Direction::W;
                }
            }
            Direction::E => {
                if clockwise {
                    *self = Direction::S;
                } else {
                    *self = Direction::N;
                }
            }
            Direction::S => {
                if clockwise {
                    *self = Direction::W;
                } else {
                    *self = Direction::E;
                }
            }
            Direction::W => {
                if clockwise {
                    *self = Direction::N;
                } else {
                    *self = Direction::S;
                }
            }
        }
    }

    pub fn rotate_dimensions(&self, dim: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N | Direction::S => {return dim}
            Direction::E | Direction::W => {return (dim.1, dim.0)}
        }
    }

    pub fn mirror(&self) -> Direction {
        return match self {
            Direction::N => {Direction::S}
            Direction::E => {Direction::W}
            Direction::S => {Direction::N}
            Direction::W => {Direction::E}
        }
    }

    pub fn angle(&self) -> Angle {
        match self {
            Direction::N => {return Angle::half_pi()}
            Direction::E => {return Angle::zero()}
            Direction::S => {return Angle::three_halves_pi()}
            Direction::W => {return Angle::pi()}
        }
    }

    pub fn normal_vector(&self) -> D2<D1> {
        match self {
            Direction::N => {return D2::from((0, 1))}
            Direction::E => {return D2::from((1, 0))}
            Direction::S => {return D2::from((0, -1))}
            Direction::W => {return D2::from((-1, 0))}
        }
    }
    pub fn normal_vector_pos(&self) -> D2<u16> {
        match self {
            Direction::N => {return (0, 1).into()}
            Direction::E => {return (1, 0).into()}
            Direction::S => {return (0, 1).into()}
            Direction::W => {return (1, 0).into()}
        }
    }
    pub fn horizontal(&self) -> bool {
        match self {
            Direction::N | Direction::S => {return false}
            Direction::E | Direction::W => {return true}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Inclusive {
    Inclusive,
    Exclusive,
    Half,
}

pub trait Corners<T: BasicMath> {
    fn center(self) -> D2<T>;
    fn bottom_right(&self) -> D2<T>;
    fn top_left(&self) -> D2<T>;
    fn dim(&self) -> D2<T>;
    fn pull_side(&self, direction: Direction, amount: T) -> Self;
    fn pull_all_sides(&self, amount: T) -> Self;
    fn split_in_halves(&self) -> [(D2<T>, D2<T>); 4];
    fn split_in_halve(&self, horizontal: bool) -> [(D2<T>, D2<T>); 2];
    fn fix_corners(self) -> Self;
    fn is_surrounded_by_rect(&self, big_corners: (D2<T>, D2<T>), inclusive: Inclusive) -> bool;
    fn contains_pt(&self, point: D2<T>, inclusive: Inclusive) -> bool;
}
impl<T: BasicMath> Corners<T> for (D2<T>, D2<T>) {
    fn center(self) -> D2<T> {
        self.0 + (self.1 - self.0) / D2::from(T::from(2))
    }
    fn bottom_right(&self) -> D2<T> {
        D2::from((self.1.x, self.0.y))
    }
    fn top_left(&self) -> D2<T> {
        D2::from((self.0.x, self.1.y))
    }
    fn dim(&self) -> D2<T> {
        D2::from((self.1.x - self.0.x, self.1.y - self.0.y))
    }
    /// positive amount pulls corners outwards
    fn pull_side(&self, direction: Direction, amount: T) -> Self {
        let mut new = self.clone();
        match direction {
            Direction::N => {
                new.1.y = new.1.y + amount;
            }
            Direction::S => {
                new.0.y = new.0.y - amount;
            }
            Direction::E => {
                new.1.x = new.1.x + amount;
            }
            Direction::W => {
                new.0.x = new.0.x - amount;
            }
        }
        return new
    }
    /// positive amount pulls corners outwards
    fn pull_all_sides(&self, amount: T) -> Self {
        let mut new = self.clone();
        new.0.x = new.0.x - amount;
        new.0.y = new.0.y - amount;
        new.1.x = new.1.x + amount;
        new.1.y = new.1.y + amount;
        return new
    }
    fn split_in_halve(&self, horizontal: bool) -> [(D2<T>, D2<T>); 2] {
        let x1 = self.0.x;
        let y1 = self.0.y;
        let x2 = self.1.x;
        let y2 = self.1.y;
        let x_div = (x2 - x1) / 2.into();
        let y_div = (y2 - y1) / 2.into();
        if horizontal {
            let top = (D2::from((x1, y1 + y_div)), D2::from((x2, y2)));
            let bottom = (D2::from((x1, y1)), D2::from((x2, y1 + y_div)));
            return [top, bottom]
        } else {
            let right = (D2::from((x1 + x_div, y1)), D2::from((x2, y2)));
            let left = (D2::from((x1, y1)), D2::from((x1 + x_div, y2)));
            return [right, left]
        }
    }
    fn split_in_halves(&self) -> [(D2<T>, D2<T>); 4] {
        let x1 = self.0.x;
        let y1 = self.0.y;
        let x2 = self.1.x;
        let y2 = self.1.y;
        let x_div = (x2 - x1) / 2.into();
        let y_div = (y2 - y1) / 2.into();

        let ne = (D2::from((x1 + x_div, y1 + y_div)), D2::from((x2, y2)));
        let se = (D2::from((x1 + x_div, y1)), D2::from((x2, y1 + y_div)));   
        let sw = (D2::from((x1, y1)), D2::from((x1 + x_div, y1 + y_div)));
        let nw = (D2::from((x1, y1 + y_div)), D2::from((x1 + x_div, y2)));
        [ne, se, sw, nw]
    }
    /// fix_corners makes sure that the first point is the bottom left of the box and
    /// the second point is the top right
    fn fix_corners(self) -> Self {
        let x1;
        let x2;
        let y1;
        let y2;
        if self.0 .x > self.1 .x {
            x1 = self.1 .x;
            x2 = self.0 .x;
        } else {
            x1 = self.0 .x;
            x2 = self.1 .x;
        }
        if self.0 .y > self.1 .y {
            y1 = self.1 .y;
            y2 = self.0 .y;
        } else {
            y1 = self.0 .y;
            y2 = self.1 .y;
        }
        return (D2::from((x1, y1)), D2::from((x2, y2)))
    }
    // is small rect completely contained by big rect?
    fn is_surrounded_by_rect(&self, big_corners: (D2<T>, D2<T>), inclusive: Inclusive) -> bool {
        match inclusive {
            Inclusive::Inclusive => {
                if self.0.x >= big_corners.0.x
                    && self.0.y >= big_corners.0.y
                    && self.1.x <= big_corners.1.x
                    && self.1.y <= big_corners.1.y
                    {return true}
            }
            Inclusive::Exclusive => {
                if self.0.x > big_corners.0.x
                    && self.0.y > big_corners.0.y
                    && self.1.x < big_corners.1.x
                    && self.1.y < big_corners.1.y
                    {return true}
            }
            Inclusive::Half => {
                if self.0.x >= big_corners.1.x
                    || self.0.y >= big_corners.1.y
                    || self.1.x < big_corners.0.x
                    || self.1.y < big_corners.0.y
                    {return true}
                }
            }
        return false
    }
    fn contains_pt(&self, point: D2<T>, inclusive: Inclusive) -> bool {
        match inclusive {
            Inclusive::Inclusive => {
                if point.x >= self.0.x
                && point.x <= self.1.x
                && point.y >= self.0.y
                && point.y <= self.1.y {
                    return true
                }
            }
            Inclusive::Exclusive => {
                if point.x > self.0.x
                && point.x < self.1.x
                && point.y > self.0.y
                && point.y < self.1.y {
                    return true
                }
            }
            Inclusive::Half => {
                if point.x >= self.0.x
                && point.x < self.1.x
                && point.y >= self.0.y
                && point.y < self.1.y {
                    return true
                }
            }
        }
        false
    }

}

pub fn does_rect_rect_intersect<T: BasicMath>(a: (D2<T>, D2<T>), b: (D2<T>, D2<T>), inclusive: Inclusive) -> bool {
    match inclusive {
        Inclusive::Inclusive => {
            if a.1.x < b.0.x || a.1.y < b.0.y || a.0.x > b.1.x || a.0.y > b.1.y {
                return false
            }
        }
        Inclusive::Exclusive | Inclusive::Half => {
            if a.1.x <= b.0.x || a.1.y <= b.0.y || a.0.x >= b.1.x || a.0.y >= b.1.y {
                return false
            }
        }
    }
    true
}

pub fn does_rect_circle_intersect<T: Stuff>(rect: (D2<T>, D2<T>), circle: (D2<T>, T), inclusive: Inclusive) -> bool {
    let (center, radius) = circle;
    // clamp center of circle to rectangle
    let clamped = D2::from((
        center.x.max(rect.0.x).min(rect.1.x),
        center.y.max(rect.0.y).min(rect.1.y),
    ));
    match inclusive {
        Inclusive::Inclusive => {
            if center.distance(clamped) <= radius {
                return true
            }
        }
        Inclusive::Exclusive | Inclusive::Half => {
            if center.distance(clamped) < radius {
                return true
            }
        }
    }
    return false
}

/// pointing from rect to circle (not tested)
/// not sure what happens if circle center is inside rect
/// should only ever return Some if [does_rect_circle_intersect_inclusive] returns true (if there is a physical collision)
pub fn rect_circle_collision_normal<T: Stuff + Neg<Output = T>>(rect: (D2<T>, D2<T>), circle: (D2<T>, T)) -> Option<D2<T>> {
    let (center, radius) = circle;
    // clamp center of circle to rectangle
    let clamped = D2::from((
        center.x.max(rect.0.x).min(rect.1.x),
        center.y.max(rect.0.y).min(rect.1.y),
    ));
    let distance = center.distance(clamped);
    if distance < radius {
        return Some((clamped - center).normalize())
    }
    return None
}

/// returns vector with length of 1 from a to b
/// not tested
/// should only ever return Some if [does_rect_rect_intersect_inclusive] returns true (if there is a physical collision)
pub fn rect_rect_collision_normal<T: Stuff + Neg<Output = T>>(a: (D2<T>, D2<T>), b: (D2<T>, D2<T>)) -> Option<D2<T>> {
    if does_rect_rect_intersect(a, b, Inclusive::Exclusive) {
        let a_center = a.center();
        let b_center = b.center();
        let diff = a_center - b_center;
        let mut normal = D2::from((T::zero(), T::zero()));
        if diff.x.abs() > diff.y.abs() {
            if diff.x > T::zero() {
                normal.x = T::one();
            } else {
                normal.x = -(T::one());
            }
        } else {
            if diff.y > T::zero() {
                normal.y = T::one();
            } else {
                normal.y = -(T::one());
            }
        }
        return Some(normal)
    }
    return None
}

pub fn does_circle_circle_intersect<T: Stuff>(a: (D2<T>, T), b: (D2<T>, T), inclusive: Inclusive) -> bool {
    let (center_a, radius_a) = a;
    let (center_b, radius_b) = b;
    let distance = center_a.distance(center_b);
    match inclusive {
        Inclusive::Inclusive => {
            if distance <= radius_a + radius_b {
                return true
            }
        }
        Inclusive::Exclusive | Inclusive::Half => {
            if distance < radius_a + radius_b {
                return true
            }
        }
    }
    return false
}

/// returns vector with length of 1 from a to b
/// should only ever return Some if [does_circle_circle_intersect_inclusive] returns true (if there is a physical collision)
pub fn circle_circle_collision_normal<T: Stuff + Neg<Output = T>>(a: (D2<T>, T), b: (D2<T>, T)) -> Option<D2<T>> {
    let (center_a, radius_a) = a;
    let (center_b, radius_b) = b;
    let distance = center_a.distance(center_b);
    if distance < radius_a + radius_b {
        return Some((center_a - center_b).normalize())
    }
    return None
}

pub fn does_prism_prism_intersect(a: (D3, D3), b: (D3, D3), inclusive: Inclusive) -> bool {
    match inclusive {
        Inclusive::Inclusive => {
            if a.1.x < b.0.x || a.1.y < b.0.y || a.1.z < b.0.z || a.0.x > b.1.x || a.0.y > b.1.y || a.0.z > b.1.z {
                return false
            }
        }
        Inclusive::Exclusive | Inclusive::Half => {
            if a.1.x <= b.0.x || a.1.y <= b.0.y || a.1.z <= b.0.z || a.0.x >= b.1.x || a.0.y >= b.1.y || a.0.z >= b.1.z {
                return false
            }
        }
    }
    true
}

pub fn does_prism_sphere_intersect(rect: (D3, D3), circle: (D3, D1), inclusive: Inclusive) -> bool {
    let (center, radius) = circle;
    // clamp center of circle to rectangle
    let clamped = D3::from((
        center.x.max(rect.0.x).min(rect.1.x),
        center.y.max(rect.0.y).min(rect.1.y),
        center.z.max(rect.0.z).min(rect.1.z),
    ));
    match inclusive {
        Inclusive::Inclusive => {
            if center.distance(clamped) <= radius {
                return true
            }
        }
        Inclusive::Exclusive | Inclusive::Half => {
            if center.distance(clamped) < radius {
                return true
            }
        }
    }
    return false
}

pub fn is_prism_surrounded(small: (D3, D3), big: (D3, D3), inclusive: Inclusive) -> bool {
    match inclusive {
        Inclusive::Inclusive => {
            if small.0.x >= big.0.x
                && small.0.y >= big.0.y
                && small.0.z >= big.0.z
                && small.1.x <= big.1.x
                && small.1.y <= big.1.y
                && small.1.z <= big.1.z
                {return true}
        }
        Inclusive::Exclusive => {
            if small.0.x > big.0.x
                && small.0.y > big.0.y
                && small.0.z > big.0.z
                && small.1.x < big.1.x
                && small.1.y < big.1.y
                && small.1.z < big.1.z
                {return true}
        }
        Inclusive::Half => {
            if small.0.x >= big.1.x
                || small.0.y >= big.1.y
                || small.0.z >= big.1.z
                || small.1.x < big.0.x
                || small.1.y < big.0.y
                || small.1.z < big.0.z
                {return true}
            }
        }
    return false
}



pub trait CornerStuff3D {
    fn dimensions(&self) -> D3;
    fn center(&self) -> D3;
    fn to_2d(&self) -> (D2<D1>, D2<D1>);
    fn contains_pt(&self, point: D3, inclusive: Inclusive) -> bool;
}
impl CornerStuff3D for (D3, D3) {

    fn dimensions(&self) -> D3 {
        D3::from((self.1.x - self.0.x, self.1.y - self.0.y, self.1.z - self.0.z))
    }
    fn center(&self) -> D3 {
        self.0 + (self.1 - self.0) / D3::from(2)
    }
    /// drops the z dimension from 3d corners and returns 2d corners
    fn to_2d(&self) -> (D2<D1>, D2<D1>) {
        (D2::from((self.0.x, self.0.y)), D2::from((self.1.x, self.1.y)))
    }
    fn contains_pt(&self, point: D3, inclusive: Inclusive) -> bool {
        match inclusive {
            Inclusive::Inclusive => {
                if point.x >= self.0.x
                && point.x <= self.1.x
                && point.y >= self.0.y
                && point.y <= self.1.y
                && point.z >= self.0.z
                && point.z <= self.1.z {
                    return true
                }
            }
            Inclusive::Exclusive => {
                if point.x > self.0.x
                && point.x < self.1.x
                && point.y > self.0.y
                && point.y < self.1.y
                && point.z > self.0.z
                && point.z < self.1.z {
                    return true
                }
            }
            Inclusive::Half => {
                if point.x >= self.0.x
                && point.x < self.1.x
                && point.y >= self.0.y
                && point.y < self.1.y
                && point.z >= self.0.z
                && point.z < self.1.z {
                    return true
                }
            }
        }
        false
    }
}
















#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape2D<T> {
    Rect((D2<T>, D2<T>)), // hitbox is a rectangle
    Circle((D2<T>, T)), // coords and radius
}
impl<T: Stuff> Shape2D<T> {
    pub fn move_center_to(&self, pt: D2<T>) -> Shape2D<T> {
        match self {
            Shape2D::Rect(corners) => {
                let dim = corners.dim();
                let corners = (
                    pt - dim/D2::from(T::from(2)),
                    pt + dim/D2::from(T::from(2)),
                ).fix_corners();
                Shape2D::Rect(corners)
            }
            Shape2D::Circle((_, radius)) => Shape2D::Circle((pt, *radius)),
        }
    }
    pub fn corners(&self) -> (D2<T>, D2<T>) {
        return match self {
            Shape2D::Rect(corners) => (*corners).clone(),
            Shape2D::Circle((center, radius)) => (
                D2::from(((*center).x - *radius, (*center).y - *radius)), 
                D2::from(((*center).x + *radius, (*center).y + *radius))),
        }
    }
    pub fn center(&self) -> D2<T> {
        return match self {
            Shape2D::Rect(corners) => corners.0 + (corners.1 - corners.0)/D2::from(T::from(2)),
            Shape2D::Circle((center, _)) => (*center).clone(),
        }
    }
    pub fn does_contain_point(&self, pt: D2<T>, inclusive: Inclusive) -> bool {
        match self {
            Shape2D::Rect(corners) => {
                corners.contains_pt(pt, inclusive)
            }
            Shape2D::Circle(circle) => {
                does_circle_circle_intersect(*circle, (pt, T::zero()), inclusive)
            }
        }
    }
    pub fn does_collide_with_rect(&self, corners: (D2<T>, D2<T>), inclusive: Inclusive) -> bool {
        match self {
            Shape2D::Rect(rect) => {
                does_rect_rect_intersect(*rect, corners, inclusive)
            }
            Shape2D::Circle((center, radius)) => {
                does_rect_circle_intersect(corners, (*center, *radius), inclusive)
            }
        }
    }

    pub fn does_collide_with_shape(&self, other: &Shape2D<T>, inclusive: Inclusive) -> bool {
        match self {
            Shape2D::Rect(corners) => {
                other.does_collide_with_rect(*corners, inclusive)
            }
            Shape2D::Circle((center, radius)) => {
                match other {
                    Shape2D::Rect(corners) => {
                        does_rect_circle_intersect(*corners, (*center, *radius), inclusive)
                    }
                    Shape2D::Circle(circle2) => {
                        does_circle_circle_intersect((*center, *radius), *circle2, inclusive)
                    }
                }
            }
        }
    }
}

impl<T: Stuff + Neg<Output = T>> Shape2D<T> {
    pub fn shape_collision_normal(&self, other: &Shape2D<T>) -> Option<D2<T>> {
        match self {
            Shape2D::Circle(circle) => {
                match other {
                    Shape2D::Rect(rect) => {
                        let vector = rect_circle_collision_normal(*rect, *circle);
                        if vector.is_some() {
                            return Some(-vector.unwrap())
                        }
                        return None
                    }
                    Shape2D::Circle(circle2) => {
                        circle_circle_collision_normal(*circle,* circle2)
                    }
                }
            }
            Shape2D::Rect(rect) => {
                match other {
                    Shape2D::Rect(rect2) => {
                        rect_rect_collision_normal(*rect, *rect2)
                    }
                    Shape2D::Circle(circle) => {
                        rect_circle_collision_normal(*rect, *circle)
                    }
                }
            }
        }
    }
}

impl Shape2D<D1> {
    pub fn transform(&self, from_pos: Pos, to_pos: Pos) -> Shape2D<D1> {
        let from = from_pos.xy();
        let angle_from = from_pos.angle;
        let to = to_pos.xy();
        let angle_to = to_pos.angle;
        match self {
            Shape2D::Rect(corners) => {
                let center = corners.center();
                let dim = corners.dim();
                let center = center.transform(from, angle_from, to, angle_to);
                let corners = (
                    D2::from((center.x - dim.x/2.into(), center.y - dim.y/2.into())),
                    D2::from((center.x + dim.x/2.into(), center.y + dim.y/2.into())),
                ).fix_corners();
                Shape2D::Rect(corners)
            }
            Shape2D::Circle((center, radius)) => {
                let center = center.transform(from, angle_from, to, angle_to);
                Shape2D::Circle((center, *radius))
            }
        }
    }
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Shape3D {
    Point(D3), // hitbox is a point
    RectPrism((D3, D3)), // hitbox is a rectangle
    Sphere((D3, D1)), // coords and radius
}
impl Shape3D {
    pub fn hitbox(&self) -> Shape2D<D1> {
        match self {
            Shape3D::Point(pt) => Shape2D::Circle((pt.xy(), D1::zero())),
            Shape3D::RectPrism((pt1, pt2)) => Shape2D::Rect((pt1.xy(), pt2.xy())),
            Shape3D::Sphere((pt, rad)) => Shape2D::Circle((pt.xy(), *rad)),
        }
    }
    pub fn center(&self) -> D3 {
        match self {
            Shape3D::Point(pt) => *pt,
            Shape3D::RectPrism((pt1, pt2)) => {D3::from((
                (pt2.x - pt1.x) / D1::from(2),
                (pt2.y - pt1.y) / D1::from(2),
                (pt2.z - pt1.z) / D1::from(2)
            )) + *pt1},
            Shape3D::Sphere((pt, _)) => *pt
        }
    }
    pub fn corners(&self) -> (D3, D3) {
        match self {
            Shape3D::Point(pt) => (*pt, *pt),
            Shape3D::RectPrism((pt1, pt2)) => (*pt1, *pt2),
            Shape3D::Sphere((pt, rad)) => {
                let rad = *rad;
                let pt1 = D3::from((pt.x - rad, pt.y - rad, pt.z - rad));
                let pt2 = D3::from((pt.x + rad, pt.y + rad, pt.z + rad));
                (pt1, pt2)
            }
        }
    }
    pub fn contains_point_inclusive(&self, pt: D3) -> bool {
        match self {
            Shape3D::Point(pt1) => pt == *pt1,
            Shape3D::RectPrism(prism) => {
                prism.contains_pt(pt, Inclusive::Inclusive)
            }
            Shape3D::Sphere((pt1, rad)) => {
                let rad = *rad;
                let dx = pt.x - pt1.x;
                let dy = pt.y - pt1.y;
                let dz = pt.z - pt1.z;
                dx*dx + dy*dy + dz*dz <= rad*rad
            }
        }
    }
    pub fn collides_with_prism(&self, corners: (D3, D3), inclusive: Inclusive) -> bool {
        match self {
            Shape3D::Point(pt) => {
                return corners.contains_pt(*pt, inclusive)
            }
            Shape3D::RectPrism(rect) => {
                does_prism_prism_intersect(*rect, corners, inclusive)
            }
            Shape3D::Sphere((center, radius)) => {
                does_prism_sphere_intersect(corners, (*center, *radius), inclusive)
            }
        }
    }
    pub fn transform(&self, from_pos: Pos, to_pos: Pos) -> Shape3D {
        match self {
            Shape3D::Point(pt) => {
                let pt = pt.transform(from_pos, to_pos);
                Shape3D::Point(pt)
            }
            Shape3D::RectPrism((pt1, pt2)) => {
                let pt1 = pt1.transform(from_pos, to_pos);
                let pt2 = pt2.transform(from_pos, to_pos);
                Shape3D::RectPrism((pt1, pt2))
            }
            Shape3D::Sphere((pt, radius)) => {
                let pt = pt.transform(from_pos, to_pos);
                Shape3D::Sphere((pt, *radius))
            }
        }
    }
}
