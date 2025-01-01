use std::{
    cmp::{max, min},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    m: Vec<T>,
    len: (usize, usize),
}

impl<T> Vec2d<T> {
    pub const fn new() -> Self {
        Vec2d {
            m: Vec::new(),
            len: (0, 0),
        }
    }

    pub fn with_capacity(cap: (usize, usize)) -> Self {
        Vec2d {
            m: Vec::with_capacity(cap.0 * cap.1),
            len: (0, 0),
        }
    }

    /// Creates a new `Vec2d` with given dimensions and a filler (which is
    /// called with the `x` and `y` coordinates)
    pub fn new_with_fill<F: Fn((usize, usize)) -> T>(len: (usize, usize), fill: F) -> Self {
        let mut v = Vec2d::with_capacity(len);
        for y in 0..len.1 {
            for x in 0..len.0 {
                v.m.push(fill((x, y)));
            }
        }
        v.len = len;
        v
    }

    #[inline]
    pub fn len(&self) -> (usize, usize) {
        self.len
    }

    pub fn get(&self, i: (usize, usize)) -> Option<&T> {
        let len = self.len();
        self.m.get(i.0.wrapping_add(i.1.wrapping_mul(len.0)))
    }

    pub fn get_mut(&mut self, i: (usize, usize)) -> Option<&mut T> {
        let len = self.len();
        if i.0 >= len.0 || i.1 >= len.1 {
            None
        } else {
            self.m.get_mut(i.0.wrapping_add(i.1.wrapping_mul(len.0)))
        }
    }

    /// This can only fail if `self.len() == (0, 0)`
    pub fn get_clamped(&self, i: (isize, isize)) -> Option<&T> {
        let len = (self.len().0 as isize, self.len().1 as isize);
        self.m.get(
            i.0.clamp(0, len.0 - 1)
                .wrapping_add(i.1.clamp(0, len.1 - 1).wrapping_mul(len.0)) as usize,
        )
    }

    pub fn get_mut_clamped(&mut self, i: (isize, isize)) -> Option<&mut T> {
        let len = (self.len().0 as isize, self.len().1 as isize);
        self.m.get_mut(
            i.0.clamp(0, len.0 - 1)
                .wrapping_add(i.1.clamp(0, len.1 - 1).wrapping_mul(len.0)) as usize,
        )
    }

    /// Returns a reference to `self` as a flat one dimensional slice in
    /// `self.len.1` major order
    pub fn get_flat1(&self) -> &[T] {
        &self.m
    }

    pub fn get_mut_flat1(&mut self) -> &mut [T] {
        &mut self.m
    }

    pub fn for_each<F: Fn(&mut T, (usize, usize))>(&mut self, f: F) {
        for y in 0..self.len().1 {
            for x in 0..self.len().0 {
                f(self.get_mut((x, y)).unwrap(), (x, y));
            }
        }
    }
}

impl<T: Copy> Vec2d<T> {
    /// Scales `v` by `scale` times
    pub fn new_scaled_from(v: &Vec2d<T>, scale: (usize, usize)) -> Self {
        let len = (scale.0 * v.len.0, scale.1 * v.len.1);
        let mut v_scaled = Vec2d::with_capacity(len);
        for y in 0..v.len.1 {
            for _ in 0..scale.1 {
                for x in 0..v.len.0 {
                    for _ in 0..scale.0 {
                        v_scaled.m.push(v[(x, y)]);
                    }
                }
            }
        }
        v_scaled.len = len;
        v_scaled
    }

    /// Adds a slice oriented such that `self.len.1` is incremented
    ///
    /// # Panics
    ///
    /// - If `slice0.len() != self.len.0`
    pub fn push1(&mut self, slice0: &[T]) {
        assert_eq!(slice0.len(), self.len.0);
        self.m.extend_from_slice(slice0);
        self.len.1 += 1;
    }

    pub fn copy_to_middle_from_vec2d_cropped(&mut self, rhs: &Vec2d<T>) {
        // the middle of `self`
        /*let self_mid = (self.len().0 >> 1, self.len().1 >> 1);
        let rhs_mid = (rhs.len().0 >> 1, rhs.len().1 >> 1);
        self.copy_from_vec2d_cropped(
            rhs,
            (
                (self_mid.0 as isize) - (rhs_mid.0 as isize),
                (self_mid.1 as isize) - (rhs_mid.1 as isize),
            ),
        );*/
        self.copy_from_vec2d_cropped(
            rhs,
            (
                ((self.len().0 as isize) - (rhs.len().0 as isize)) / 2,
                ((self.len().1 as isize) - (rhs.len().1 as isize)) / 2,
            ),
        );
    }

    /// Copies `rhs` to `self` at location `loc`. If any part of `rhs` would be
    /// copied out of bounds on `self`, it just gets cropped.
    pub fn copy_from_vec2d_cropped(&mut self, rhs: &Vec2d<T>, loc: (isize, isize)) {
        if self.len().0 == 0 {
            return
        }
        let min_i = (
            max(0, loc.0.wrapping_neg()) as usize,
            max(0, loc.1.wrapping_neg()) as usize,
        );

        let max_i = (
            min(
                rhs.len().0 as isize,
                (self.len().0 as isize).wrapping_sub(loc.0),
            ) as usize,
            min(
                rhs.len().1 as isize,
                (self.len().1 as isize).wrapping_sub(loc.1),
            ) as usize,
        );

        for y in min_i.1..max_i.1 {
            for x in min_i.0..max_i.0 {
                self[(
                    x.wrapping_add(loc.0 as usize),
                    y.wrapping_add(loc.1 as usize),
                )] = rhs[(x, y)];
            }
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &T {
        self.get(i).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut T {
        self.get_mut(i).unwrap()
    }
}
