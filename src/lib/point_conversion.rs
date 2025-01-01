
use fixed::traits::ToFixed;
use fixed::types::I32F32;
use fixed::types::I52F12;
use fixed::types::I36F28;
use common::*;



// impl From<I52F12> for D1 {
//     fn from(val: I52F12) -> Self {Self {val}}
// }
// impl From<i32> for D1 {
//     fn from(val: i32) -> Self {Self {val: I52F12::from_num(val)}}
// }
// impl From<u32> for D1 {
//     fn from(val: u32) -> Self {Self {val: I52F12::from_num(val)}}
// }
// impl From<u16> for D1 {
//     fn from(val: u16) -> Self {Self {val: I52F12::from_num(val)}}
// }
// impl From<f32> for D1 {
//     fn from(val: f32) -> Self {Self {val: I52F12::from_num(val)}}
// }
// impl From<usize> for D1 {
//     fn from(val: usize) -> Self {Self {val: I52F12::from_num(val as u32)}}
// }
impl From<Angle> for D1 {
    fn from(val: Angle) -> Self {Self {val: I36F28::from_num(val.val)}}
}


impl<T: ToFixed> From<T> for D1 {
    fn from(t: T) -> Self {Self {val: I36F28::from_num(t)}}
}
// impl<T: ToFixed> From<(T, T)> for D2<D1> {
//     fn from(coords: (T, T)) -> Self {
//         Self {
//             x: D1::from(coords.0),
//             y: D1::from(coords.1),
//         }
//     }
// }
// impl<T: Copy + ToFixed> From<T> for D2<D1> {
//     fn from(val: T) -> Self {
//         Self {
//             x: D1::from(val),
//             y: D1::from(val),
//         }
//     }
// }

impl<T> From<(T, T)> for D2<T> {
    fn from(coords: (T, T)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
        }
    }
}
impl<T: Copy> From<T> for D2<T> {
    fn from(val: T) -> Self {
        Self {
            x: val,
            y: val,
        }
    }
}


impl From<(I52F12, I52F12)> for D2<D1> {
    fn from(coords: (I52F12, I52F12)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(i32, i32)> for D2<D1> {
    fn from(coords: (i32, i32)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(u32, u32)> for D2<D1> {
    fn from(coords: (u32, u32)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(u16, u16)> for D2<D1> {
    fn from(coords: (u16, u16)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(u8, u8)> for D2<D1> {
    fn from(coords: (u8, u8)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(f32, f32)> for D2<D1> {
    fn from(coords: (f32, f32)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<(usize, usize)> for D2<D1> {
    fn from(coords: (usize, usize)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
        }
    }
}
impl From<D2<u16>> for D2<D1> {
    fn from(coords: D2<u16>) -> Self {
        Self {
            x: D1::from(coords.x),
            y: D1::from(coords.y),
        }
    }
}
impl From<usize> for D2<D1> {
    fn from(val: usize) -> Self {
        Self {x: D1::from(val), y: D1::from(val)}
    }
}

impl From<(usize, usize)> for D2<u16> {
    fn from(val: (usize, usize)) -> Self {
        Self {x: val.0 as u16, y: val.1 as u16}
    }
}
impl From<(i32, i32)> for D2<u16> {
    fn from(val: (i32, i32)) -> Self {
        Self {x: val.0 as u16, y: val.1 as u16}
    }
}
impl From<D2<D1>> for D2<u16> {
    fn from(coords: D2<D1>) -> Self {
        Self {x: coords.x.to_u16(), y: coords.y.to_u16()}
    }
}

impl From<(i32, i32, i32)> for D3 {
    fn from(coords: (i32, i32, i32)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
            z: D1::from(coords.2),
        }
    }
}
impl From<(D1, D1, D1)> for D3 {
    fn from(coords: (D1, D1, D1)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }
}
impl From<(D2<D1>, D1)> for D3 {
    fn from(coords: (D2<D1>, D1)) -> Self {
        Self {
            x: coords.0.x,
            y: coords.0.y,
            z: coords.1,
        }
    }
}
impl From<(f32, f32, f32)> for D3 {
    fn from(coords: (f32, f32, f32)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
            z: D1::from(coords.2),
        }
    }
}
impl From<(usize, usize, usize)> for D3 {
    fn from(coords: (usize, usize, usize)) -> Self {
        Self {
            x: D1::from(coords.0),
            y: D1::from(coords.1),
            z: D1::from(coords.2),
        }
    }
}
impl From<usize> for D3 {
    fn from(val: usize) -> Self {
        Self {x: D1::from(val), y: D1::from(val), z: D1::from(val)}
    }
}
impl From<i32> for D3 {
    fn from(val: i32) -> Self {
        Self {x: D1::from(val), y: D1::from(val), z: D1::from(val)}
    }
}
impl From<D1> for D3 {
    fn from(val: D1) -> Self {
        Self {x: val, y: val, z: val}
    }
}
impl From<(D3, Angle)> for Pos {
    fn from((coords, angle): (D3, Angle)) -> Self {
        Self {
            coords,
            angle,
        }
    }
}
//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
impl From<f32> for Angle {
    fn from(val: f32) -> Self {Self {val: I32F32::from_num(val)}}
}
impl From<&str> for Angle {
    fn from(val: &str) -> Self {Self {val: I32F32::lit(val)}}
}
impl From<D1> for Angle {
    fn from(val: D1) -> Self {Self::from(val.to_f32())}
}
impl From<I32F32> for Angle {
    fn from(val: I32F32) -> Self {Self {val}}
}

//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA