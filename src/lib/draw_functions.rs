use std::fmt::Debug;
use std::mem::swap;

use serde::{Serialize, Deserialize};
// use common::*;
use crate::vec2d::Vec2d;
use crate::cam_data::*;
use crate::coordinates::*;
use crate::shape::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, alpha: 255 }
    }
    pub fn new_with_alpha(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self { r, g, b, alpha }
    }
    pub fn new_transparent() -> Self {
        Self { r: 0, g: 0, b: 0, alpha: 0 }
    }
    pub fn new_from_u32(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
            alpha: 255,
        }
    }
    pub fn new_from_u8(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
            alpha: 255,
        }
    }
    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) + ((self.g as u32) << 8) + (self.b as u32)
    }
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    pub fn next_shade(&mut self) {
        self.r += 10;
    }
    pub fn make_transparent(self) -> Color {
        return Color::new_with_alpha(self.r, self.g, self.b, self.alpha/2)
    }
    pub fn button_pressed_change(self) -> Color {
        return Color::new(self.r/2, self.g/2, self.b/2)
    }
    pub fn maroon() -> Self {
        Self::new(128, 0, 0)
    }
    pub fn green() -> Self {
        Self::new(0, 150, 0)
    }
    pub fn dark_green() -> Self {
        Self::new(0, 100, 0)
    }
    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }
    /// Leah's favorite color
    pub fn ultramarine_blue() -> Self {
        Self::new(65,102,245)
    }
    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }
    pub fn gray() -> Self {
        Self::new(128, 128, 128)
    }
    pub fn purple() -> Self {
        Self::new(128, 0, 128)
    }
    pub fn pink() -> Self {
        Self::new(255, 105, 180)
    }
    pub fn orange() -> Self {
        Self::new(255, 165, 0)
    }
    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }
    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    /// cycles through a list of colors. Changes self to the next color in the list
    pub fn next(self) -> Self {
        if self == Color::ultramarine_blue() {
            Color::maroon()
        } else if self == Color::maroon() {
            Color::red()
        } else if self == Color::red() {
            Color::dark_green()
        } else if self == Color::dark_green() {
            Color::green()
        } else if self == Color::green() {
            Color::blue()
        } else {
            Color::ultramarine_blue()
        }
    }
    /// merges self and other. Percent is percent of other i think but dont quote me on that
    pub fn merge(self, other: Color, percent: D1) -> Color {
        let r = (self.r as i32 + ((D1::from(other.r) - D1::from(self.r)) * percent).to_i32()) as u8;
        let g = (self.g as i32 + ((D1::from(other.g) - D1::from(self.g)) * percent).to_i32()) as u8;
        let b = (self.b as i32 + ((D1::from(other.b) - D1::from(self.b)) * percent).to_i32()) as u8;
        Color::new(r, g, b)
    }
}


pub type Bez = ((usize, usize), (usize, usize));



#[derive(Debug)]
pub struct Cam<'a> {
    pub zero_pos: Pos, // position of draw plane relative to center of screen
    pub trig_mult: TrigMult,
    pub cam_data: &'a CamData,
    pub img: &'a mut Vec2d<u32>,
    pub alpha: u8,
}
impl Cam<'_> {
    /// returns None if the z height of the draw plane is at or above the focal point of the camera (above the camera, otherwise divide by 0 errors will happen)
    pub fn new_rel_to_pos<'a>(
        img: &'a mut Vec2d<u32>,
        cam_data: &'a CamData,
        rel_to: Pos,
        alpha: u8,
    ) -> Option<Cam<'a>> {
        let zero_pos = cam_data.zero_pos(rel_to);

        if zero_pos.z() >= cam_data.fisheye() {
            return None
        }
        let cam = Cam {
            zero_pos,
            trig_mult: TrigMult::new(-zero_pos.angle),
            img,
            alpha,
            cam_data,
        };
        return Some(cam)
    }
    
    /// change draw plane without having to make a new cam
    pub fn change_cam_perspective(&mut self, zero_position: Pos) {
        if zero_position.z() >= self.cam_data.fisheye() {
            //println!("this would have been a crash");
            return
        }
        self.zero_pos = zero_position;
        self.trig_mult = TrigMult::new(-self.zero_pos.angle);
    }
    
    //-----------
    /// takes physical coords rel to camera and returns pixel coords rel to bottom left of screen
    pub fn coords_to_cam_coords(&self, coords: D2<D1>) -> (usize, usize) {
        let coords = coords.rotate_with_trig_mult(self.trig_mult);

        let rel_point = D3::from((
            (coords.x + self.zero_pos.x())/self.cam_data.zoom(),
            (coords.y + self.zero_pos.y())/self.cam_data.zoom(),
            self.zero_pos.z()));


        let cam_fish = self.cam_data.fisheye();
        let x = rel_point.x * cam_fish / (-rel_point.z+cam_fish);
        let y = rel_point.y * cam_fish / (-rel_point.z+cam_fish);

        // makes coords rel to center of screen instead of bottom left
        let yeet = D2::from((x + D1::from(self.cam_data.winsize().0)/D1::two(), y + D1::from(self.cam_data.winsize().1)/D1::two()));
        return yeet.to_usize()
    }
    //-----------



    pub fn draw_pixel(&mut self, coords: D2<D1>, color: Color) {
        draw_pixel(self.img, self.coords_to_cam_coords(coords), color);
    }

    pub fn draw_line(&mut self, line: (D2<D1>, D2<D1>), color: Color) {
        let pt1_cam = self.coords_to_cam_coords(line.0);
        let pt2_cam = self.coords_to_cam_coords(line.1);
        draw_bezier(self.img, (pt1_cam, pt2_cam), color);
    }

    pub fn draw_rect(
        &mut self,
        center: D2<D1>,
        width: D2<D1>,
        angle: Angle,
        color: Color,
    ) {
        let bl = D2::from((center.x - width.x, center.y - width.y));
        let tr = D2::from((center.x + width.x, center.y + width.y));
        let br = D2::from((tr.x, bl.y)); // bottom right corner
        let tl = D2::from((bl.x, tr.y)); // top left corner
        let trig_mult = TrigMult::new(-angle);
        let bl = bl.rotate_with_trig_mult(trig_mult);
        let br = br.rotate_with_trig_mult(trig_mult);
        let tr = tr.rotate_with_trig_mult(trig_mult);
        let tl = tl.rotate_with_trig_mult(trig_mult);

        self.draw_line((bl, br), color);
        self.draw_line((br, tr), color);
        self.draw_line((tr, tl), color);
        self.draw_line((tl, bl), color);
    }

    
    pub fn draw_rect_corners(&mut self, corners: (D2<D1>, D2<D1>), color: Color) {
        let bl = corners.0; // bottom left
        let tr = corners.1; // top right
        let br = D2::from((tr.x, bl.y)); // bottom right corner
        let tl = D2::from((bl.x, tr.y)); // top left corner

        self.draw_line((bl, br), color);
        self.draw_line((br, tr), color);
        self.draw_line((tr, tl), color);
        self.draw_line((tl, bl), color);
    }

    // pub fn draw_rect_corners_solid(&mut self, corners: (D2<D1>, D2<D1>), color: Color) {
    //     let bl = corners.0; // bottom left
    //     let tr = corners.1; // top right
        
    //     let (bl, tr) = (bl, tr).fix_corners();
    //     let bl = self.coords_to_cam_coords(bl);
    //     let tr = self.coords_to_cam_coords(tr);

    //     for x in bl.0..tr.0 {
    //         for y in bl.1..tr.1 {
    //             draw_pixel(self.img, (x, y), color);
    //         }
    //     }
    // }
    
    pub fn draw_wide_pixel(&mut self, coords: D2<D1>, width: usize, color: Color) {
        // bottom left
        let bl = self.coords_to_cam_coords(coords);

        for x in bl.0..(bl.0 + width) {
            for y in bl.1..(bl.1 + width) {
                draw_pixel(self.img, (x, y), color);
            }
        }
    }

    pub fn draw_rect_prism_corners(&mut self, corners: (D3, D3), color: Color) {
        let bl = corners.0.xy(); // bottom left
        let tr = corners.1.xy(); // top right
        let br = D2::from((tr.x, bl.y)); // bottom right
        let tl = D2::from((bl.x, tr.y)); // top left

        let original_zero = self.zero_pos;

        if corners.0.z < self.cam_data.cam_pos_abs.z() {
            let mut zero_position = self.zero_pos;
            zero_position.coords.z += corners.0.z;
            self.change_cam_perspective(zero_position);
            self.draw_line((bl, br), color);
            self.draw_line((br, tr), color);
            self.draw_line((tr, tl), color);
            self.draw_line((tl, bl), color);
            self.change_cam_perspective(original_zero);
        }

        if corners.1.z < self.cam_data.cam_pos_abs.z() {
            let mut zero_position = self.zero_pos;
            zero_position.coords.z += corners.1.z;
            self.change_cam_perspective(zero_position);
            self.draw_line((bl, br), color);
            self.draw_line((br, tr), color);
            self.draw_line((tr, tl), color);
            self.draw_line((tl, bl), color);
            self.change_cam_perspective(original_zero);
        }
    }


    /// circle is made up of line segments. step is the angle between each point on the circle
    pub fn draw_circle(&mut self, center: D2<D1>, radius: D1, color: Color) {
        self.draw_polygon(center, radius, 16, Angle::min(), color);
    }

    pub fn draw_arc(&mut self, center: D2<D1>, radius: D1, start: Angle, end: Angle, step: Angle, color: Color) {
        let mut point = center + D2::from((radius, D1::zero())).rotate(start);
        let mut n = start;
        loop {
            //let new_point = (center.x + n.cos() * radius, center.y + n.sin() * radius);
            let new_point = center + D2::from((radius, D1::zero())).rotate(n);
            let bez = (point, new_point);
            self.draw_line(bez, color);
            point = bez.1;
            if n > end {
                break
            }
            n += step;
        }
    }


    pub fn draw_polygon(&mut self, center: D2<D1>, radius: D1, sides: u8, start_angle: Angle, color: Color) {
        if sides == 0 {
            return
        }
        let div = Angle::from(1.0) / Angle::from(sides as f32);
        let mut angle = start_angle;
        let start_angle = angle.clone();
        angle.wrap();
        let mut point = center + D2::from((radius, D1::zero())).rotate(angle);
        loop {
            let mut wrapped_angle = angle.clone();
            wrapped_angle.wrap();
            let new_point = center + D2::from((radius, D1::zero())).rotate(wrapped_angle);

            let bez = (point, new_point);
            self.draw_line(bez, color);
            point = bez.1;

            angle += Angle::two_pi() * div;
            if angle > start_angle + Angle::two_pi() {
                break
            }
        }
    }

    pub fn draw_text(&mut self, coords:D2<D1>, string: &str, color: Color, dimensions: D2<u16>, center: bool, background: bool) {
        let translated = self.coords_to_cam_coords(coords);
        let coords_u = (translated.0 as u16, translated.1 as u16).into();
        draw_text(self.img, coords_u, string, color, dimensions, center, background);
    }

    pub fn draw_shape(&mut self, shape: Shape2D<D1>, color: Color) {
        match shape {
            Shape2D::Rect(rect) => {
                self.draw_rect_corners(rect, color)
            }
            Shape2D::Circle(circle) => {
                self.draw_circle(circle.0, circle.1, color)
            }
        }
    }
}

// bresenham's line algorithm
/// rel to bottom left of screen
pub fn draw_bezier(img: &mut Vec2d<u32>, bez: Bez, color: Color) {
    let mut x0 = bez.0 .0 as i32;
    let mut y0 = bez.0 .1 as i32;
    let x1 = bez.1 .0 as i32;
    let y1 = bez.1 .1 as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
        draw_pixel(img, (x0 as usize, y0 as usize), color);

        if x0 == x1 && y0 == y1 {
            break
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }   
}

/// also bresenham's line algorithm
/// maybe faster than other line drawing fn
pub fn draw_line(img: &mut Vec2d<u32>, bez: Bez, color: Color) {
    let dx = (bez.1 .0 as i32 - bez.0 .0 as i32).abs();
    let dy = (bez.1 .1 as i32 - bez.0 .1 as i32).abs();
    if dx > dy {
        draw_line_horizontal(img, bez, color);
    } else {
        draw_line_vertical(img, bez, color);
    }
}

pub fn draw_line_horizontal(img: &mut Vec2d<u32>, bez: Bez, color: Color) {
    let mut x0 = bez.0 .0 as i32;
    let mut y0 = bez.0 .1 as i32;
    let mut x1 = bez.1 .0 as i32;
    let mut y1 = bez.1 .1 as i32;

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let dir = if dy < 0 { -1 } else { 1 };

    if dx != 0 {
        let mut y = y0;
        let mut p = 2 * dy - dx;
        for i in 0..dx+1 {
            draw_pixel(img, ((x0 + i) as usize, y as usize), color);
            if p >= 0 {
                y += dir;
                p -= 2*dx;
            }
            p += 2*dy;
        }
    }   
}

pub fn draw_line_vertical(img: &mut Vec2d<u32>, bez: Bez, color: Color) {
    let mut x0 = bez.0 .0 as i32;
    let mut y0 = bez.0 .1 as i32;
    let mut x1 = bez.1 .0 as i32;
    let mut y1 = bez.1 .1 as i32;

    if y0 > y1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let dir = if dx < 0 { -1 } else { 1 };

    if dy != 0 {
        let mut x = x0;
        let mut p = 2 * dx - dy;
        for i in 0..dy+1 {
            draw_pixel(img, (x as usize, (y0 + i) as usize), color);
            if p >= 0 {
                x += dir;
                p -= 2*dy;
            }
            p += 2*dx;
        }
    }
}


// 0123456789
// 0123456789abcdef
// 0x00 ... 0x09 ... 0x1a ... 0x45ff

pub fn from_rgb(color: (u8, u8, u8)) -> u32 {
    (color.2 as u32) | ((color.1 as u32) << 8) | ((color.0 as u32) << 16)
}
pub fn from_rgba(color: (u8, u8, u8, u8)) -> u32 {
    (color.0 as u32) | ((color.1 as u32) << 8) | ((color.2 as u32) << 16) | ((color.3 as u32) << 24)
    //((color.0 as u32) << 24) | ((color.1 as u32) << 16) | ((color.2 as u32) << 8) | (color.3 as u32)
}

/// rel to bottom left of screen
pub fn draw_pixel(img: &mut Vec2d<u32>, coords: (usize, usize), color: Color) {
    let y = if let Some(y) = img.len().1.checked_sub(1) {
        y
    } else {
        return
    };
    let y = if let Some(y) = y.checked_sub(coords.1 as usize) {
        y
    } else {
        return
    };
    match img.get_mut((coords.0 as usize, y)) {
        Some(p) => {
            if color.alpha == 255 {
                *p = color.to_u32();
            } else {
                let new_bytes = color.to_u32().to_be_bytes();
                let old_bytes = p.to_be_bytes();
    
                let r = old_bytes[1] as f32
                    + (new_bytes[1] as f32 - old_bytes[1] as f32) / 255.0 * color.alpha as f32;
                let g = old_bytes[2] as f32
                    + (new_bytes[2] as f32 - old_bytes[2] as f32) / 255.0 * color.alpha as f32;
                let b = old_bytes[3] as f32
                    + (new_bytes[3] as f32 - old_bytes[3] as f32) / 255.0 * color.alpha as f32;
    
                *p = from_rgb((r as u8, g as u8, b as u8));
            }
        }
        None => (),
    }
}

pub fn draw_rectangle(
    img: &mut Vec2d<u32>,
    coords: D2<u16>,
    width: D2<u16>,
    color: Color,
    centered: bool,
) {
    let coords2;
    if centered {
        coords2 = coords - width / D2::from(2);
    } else {
        coords2 = coords;
    }
    for x in coords2.x..(coords2.x + width.x) {
        for y in coords2.y..(coords2.y + width.y) {
            *img.get_mut_clamped((x as isize, (img.len().1 as u16 - y - 1) as isize))
                .unwrap() = color.to_u32();
        }
    }
}

pub fn draw_rectangle_corners(
    img: &mut Vec2d<u32>,
    corners: (D2<u16>, D2<u16>),
    color: Color,
) {
    for x in corners.0.x..corners.1.x {
        for y in corners.0.y..corners.1.y {
            *img.get_mut_clamped((x as isize, (img.len().1 as u16 - y - 1) as isize))
                .unwrap() = color.to_u32();
        }
    }
}

pub fn draw_border(img: &mut Vec2d<u32>, corners: (D2<u16>, D2<u16>), color: Color) {
    let y2 = corners.0.y;
    for x2 in (corners.0.x)..(corners.1.x) {
        draw_pixel(img, (x2 as usize, y2 as usize), color);
    }
    let y2 = corners.1.y - 1;
    for x2 in (corners.0.x)..(corners.1.x) {
        draw_pixel(img, (x2 as usize, y2 as usize), color);
    }
    let x2 = corners.0.x;
    for y2 in (corners.0.y)..(corners.1.y) {
        draw_pixel(img, (x2 as usize, y2 as usize), color);
    }
    let x2 = corners.1.x - 1;
    for y2 in (corners.0.y)..(corners.1.y) {
        draw_pixel(img, (x2 as usize, y2 as usize), color);
    }
}

const LETTER_DIM: D2<u16> = D2::new_const(5, 5);
const LETTER_SPACE_WIDTH: u16 = 1;

pub fn draw_text(
    img: &mut Vec2d<u32>,
    mut coords: D2<u16>,
    string: &str,
    color: Color,
    dimensions: D2<u16>, // use (2, 3) normally
    center: bool,
    background: bool,
) {
    // turn string into vector of characters
    let char_vec: Vec<char> = string.to_lowercase().chars().collect();
    let length = char_vec.len() as u16;

    // if true draw background box
    if background {
        let box_coords = coords - dimensions * D2::from(2);
        let mut box_dimensions = dimensions * D2::from(6);

        box_dimensions.x *= length;
        box_dimensions += dimensions * D2::from(4);

        draw_rectangle(
            img,
            box_coords,
            box_dimensions,
            Color::new(50, 50, 50),
            center,
        )
    }

    // if true, center the text by offsetting the coords
    if center {
        coords = (
            coords.x - length * dimensions.x * 3,
            coords.y - dimensions.y * 5 / 2,
        ).into();
    }
    
    // its drawin time, letter_offset changes after each letter, (x,y) changes each pixel of each letter, and (x2, y2) are the actual coords of the pixel fed to the draw function.
    let mut letter_offset = (0, 0);

    for letter in char_vec {
        // deal with functional characters
        // if letter == "\n".chars().next().unwrap() { // i wonder if this crashes if you just do "\" and nothing after
        //     letter_offset.0 = 0; // reset x offset
        //     letter_offset.1 -= dimensions.y * 8; // one line down
        //     continue
        // }

        // all other characters just get drawn normally
        let letter_a = get_letter_array(letter);
        for x in 0..5usize {
            let x2 = coords.x + (x as u16)*dimensions.x + letter_offset.0;
            for y in 0..5usize {
                let y2 = coords.y + (y as u16)*dimensions.y + letter_offset.1;
                if letter_a[x][y] == true {
                    draw_rectangle(img, (x2, y2).into(), dimensions, color, center);
                }
            }
        }
        letter_offset.0 += dimensions.x * 6;
    }
}

pub fn draw_text_corners(
    img: &mut Vec2d<u32>,
    corners: (D2<u16>, D2<u16>),
    string: &str,
    color: Color,
    pixel_dim: D2<u16>, // use (2, 3) normally
    centered: (bool, bool), // true if centered in x and y dimensions
    ) {
    let mut coords = D2::from((corners.0.x, corners.1.y - pixel_dim.y*LETTER_DIM.y)); // start at top left corner of box
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let word_vec = string.split_string();

    for mut word in word_vec {
        if word == "\n" {
            // move down a line and continue to not draw anything
            coords.x = corners.0.x;
            coords.y -= word.string_dim_pixels(pixel_dim).y + LETTER_SPACE_WIDTH*pixel_dim.y;
            lines.push(current_line);
            current_line = String::new();
            continue
        }

        // single word is too long to fit on a line by itself
        if word.string_dim_pixels(pixel_dim).x > corners.dim().x {
            if coords.x > corners.0.x {
                // move down a line
                coords.x = corners.0.x;
                coords.y -= word.string_dim_pixels(pixel_dim).y + LETTER_SPACE_WIDTH*pixel_dim.y;
                lines.push(current_line);
                current_line = String::new();
            }
            let fit_len = corners.dim().x * word.len() as u16 / word.string_dim_pixels(pixel_dim).x;
            word = word.as_str().substring(0, fit_len as usize).to_string();
        } else if coords.x + word.string_dim_pixels(pixel_dim).x > corners.1.x {
            // word doesn't fit in the, move down a line
            coords.x = corners.0.x;
            coords.y -= word.string_dim_pixels(pixel_dim).y + LETTER_SPACE_WIDTH*pixel_dim.y;
            lines.push(current_line);
            current_line = String::new();
        }
        if coords.y < corners.0.y {
            break
        }

        // draw_text(img, coords, &word, color, pixel_dim, false, false);
        if current_line.len() > 0 {
            current_line.push(' ');
        }
        current_line.push_str(&word);
        coords.x += word.string_dim_pixels(pixel_dim).x + LETTER_DIM.x*pixel_dim.x;
    }
    if current_line.len() > 0 {
        lines.push(current_line);
    }

    // ------------------------------------------------------------------------------------------
    let mut coords = D2::from((corners.0.x, corners.1.y - pixel_dim.y*LETTER_DIM.y)); // start at top left corner of box
    let y_offset;
    if centered.1 {
        let all_lines_height = ((lines.len() as u16) * LETTER_DIM.y + LETTER_SPACE_WIDTH*(lines.len() as u16 - 1))*pixel_dim.y;
        y_offset = (corners.dim().y - all_lines_height) / 2;
    } else {
        y_offset = 0;
    }
    for line in lines {
        let x_offset = if centered.0 {
            (corners.dim().x - line.string_dim_pixels(pixel_dim).x) / 2
        } else {
            0
        };
        // println!("line: {:?}, {:?}-{:?}", line, corners.dim().x, line.string_dim_pixels(pixel_dim).x);
        // println!("dim of {:?} is: {:?}. at coords {:?}", line, line.string_dim_pixels(pixel_dim), coords + D2::from((x_offset, 0)));
        draw_text(img, coords + D2::from((x_offset, 0)) - D2::from((0, y_offset)), &line, color, pixel_dim, false, false);
        coords.y -= line.string_dim_pixels(pixel_dim).y + LETTER_SPACE_WIDTH*pixel_dim.y;
    }
}


// fn split_string_lines(string: &str, dim: D2<u16>, pixel_dim: D2<u16>) -> Vec<String> {
//     let mut lines = Vec::new();
//     let mut line = String::new();

//     let mut current_width = 0;
//     for c in string.chars() {
//         current_width += (LETTER_DIM.x + LETTER_SPACE_WIDTH) * pixel_dim.x;
//         if c == '\n' {
//             lines.push(line);
//             line = String::new();
//             current_width = 0;
//             continue
//         } else if current_width <= dim.x {
//             line.push(c);
//         } else {
//             lines.push(line);
//             line = String::new();
//             line.push(c);
//             current_width = LETTER_DIM.x;
//         }
//     }
//     if line.len() > 0 {
//         lines.push(line);
//     }
//     return lines
// }

use std::ops::{Bound, RangeBounds};

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
    fn remove_trailing(&self, c: char) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
    fn remove_trailing(&self, c: char) -> &str {
        let mut end = self.len();
        for (i, ch) in self.char_indices().rev() {
            if ch != c { break; }
            end = i;
        }
        &self[..end]
    }
}

pub trait PixelLen {
    fn string_dim_pixels(&self, pixel_dim: D2<u16>) -> D2<u16>;
    fn split_string(&self) -> Vec<String>;
}
impl PixelLen for &str {
    // returns the pixel dimensions of the string in one long line, does not consider newline characters
    fn string_dim_pixels(&self, pixel_dim: D2<u16>) -> D2<u16> {
        return ((self.len() as u16 * LETTER_DIM.x + (self.len() as u16 - 1) * LETTER_SPACE_WIDTH) * pixel_dim.x,
        LETTER_DIM.y * pixel_dim.y).into()
    }
    /// splits string into vector of words, separated by spaces and special characters like \n
    fn split_string(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut word = String::new();
        for c in self.chars() {
            if c == ' ' {
                if word.len() > 0 {
                    words.push(word);
                }
                word = String::new();
            } else if c == '\n' {
                if word.len() > 0 {
                    words.push(word);
                }
                word = String::new();
                words.push("\n".to_string());
            } else {
                word.push(c);
            }
        }
        words.push(word);
        return words
    }
}

impl PixelLen for String {
    fn string_dim_pixels(&self, pixel_dim: D2<u16>) -> D2<u16> {
        return ((self.len() as u16 * LETTER_DIM.x + (self.len() as u16 - 1) * LETTER_SPACE_WIDTH) * pixel_dim.x,
        LETTER_DIM.y * pixel_dim.y).into()
    }
    /// splits string into vector of words, separated by spaces
    fn split_string(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut word = String::new();
        for c in self.chars() {
            if c == ' ' {
                if word.len() > 0 {
                    words.push(word);
                }
                word = String::new();
            } else if c == '\n' {
                if word.len() > 0 {
                    words.push(word);
                }
                word = String::new();
                words.push("\n".to_string());
            } else {
                word.push(c);
            }
        }
        words.push(word);
        return words
    }
}

pub fn draw_texture(
    img: &mut Vec2d<u32>,
    mut coords: D2<u16>,
    texture_id: usize,
    color: Color,
    dimensions: D2<u16>, // use (2, 3) normally
    center: bool,
    background: bool,
) {
    if background {
        let box_coords = coords - dimensions * D2::from(2);
        let box_dimensions = dimensions * D2::from(9) + dimensions * D2::from(4);
        draw_rectangle(
            img,
            box_coords,
            box_dimensions,
            Color::new(50, 50, 50),
            center,
        )
    }
    if center {
        coords = coords - dimensions * D2::from(9) / D2::from(2);
    }

    let texture_vec = get_texture_vec(texture_id);
    for x in 0..9usize {
        let x2 = coords.x + (x as u16) * dimensions.x;
        for y in 0..9usize {
            let y2 = coords.y + (y as u16) * dimensions.y;
            if texture_vec[x][y] == true {
                draw_rectangle(img, (x2 as u16, y2 as u16).into(), dimensions, color, center);
            }
        }
    }
}


// when the function is sus
fn get_texture_vec(texture: usize) -> Vec<[bool; 9]> {
    return match texture {
        1 => vec![
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [false, false, true, true, true, true, true, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, false, true, true, true, true, true, false, false],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
        ],
        2 => vec![
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [false, true, true, true, true, true, true, true, false],
            [false, true, false, false, true, false, false, true, false],
            [false, true, false, false, true, false, false, true, false],
            [false, true, true, true, true, true, true, true, false],
            [false, true, false, false, true, false, false, true, false],
            [false, true, false, false, true, false, false, true, false],
            [false, true, true, true, true, true, true, true, false],
            [
                false, false, false, false, false, false, false, false, false,
            ],
        ],
        _ => vec![
            [true, false, false, false, false, false, false, false, true],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false,
            ],
            [false, false, false, false, false, false, false, false, true],
        ],
    }
}

//"abcdefghijklmnopqrstuvwxyz.0123456789,!?[#]():"
fn get_letter_array(letter: char) -> [[bool; 5]; 5] {
    return match letter {
        '0' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        '1' => [
            [false, false, false, false, false],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, false],
            [false, false, false, false, false],
        ],
        '2' => [
            [true, false, false, true, false],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, true, false],
        ],
        '3' => [
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ],
        '4' => [
            [false, false, true, false, false],
            [false, false, true, true, false],
            [false, false, true, false, true],
            [true, true, true, true, true],
            [false, false, true, false, false],
        ],
        '5' => [
            [true, false, true, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, false, true],
        ],
        '6' => [
            [false, true, true, true, false],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, false, true],
        ],
        '7' => [
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, false, false, true],
            [false, false, true, false, true],
            [false, false, false, true, true],
        ],
        '8' => [
            [false, true, false, true, false],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ],
        '9' => [
            [false, false, false, true, false],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, true, true, false],
        ],
        'a' => [
            [true, true, true, true, false],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [true, true, true, true, false],
        ],
        'b' => [
            [true, true, true, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ],
        'c' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'd' => [
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'e' => [
            [true, true, true, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
        ],
        'f' => [
            [true, true, true, true, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
        ],
        'g' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, false, false, true],
        ],
        'h' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, true, true, true],
        ],
        'i' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'j' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
        ],
        'k' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
        ],
        'l' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ],
        'm' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, false, false, true, false],
            [true, true, true, true, true],
        ],
        'n' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, true, true, true, true],
        ],
        'o' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'p' => [
            [true, true, true, true, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [false, false, true, false, true],
            [false, false, false, true, false],
        ],
        'q' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, true, true, true, false],
        ],
        'r' => [
            [true, true, true, true, true],
            [false, false, true, false, true],
            [false, true, true, false, true],
            [true, false, true, false, true],
            [false, false, false, true, false],
        ],
        's' => [
            [true, false, false, true, false],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, false, true],
        ],
        't' => [
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
        ],
        'u' =>[
            [false, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, true, true],
        ],
        'v' => [
            [false, false, false, true, true],
            [false, true, true, false, false],
            [true, false, false, false, false],
            [false, true, true, false, false],
            [false, false, false, true, true],
        ],
        'w' => [
            [false, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [false, true, true, true, true],
        ],
        'x' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
        ],
        'y' => [
            [false, false, false, false, true],
            [false, false, false, true, false],
            [true, true, true, false, false],
            [false, false, false, true, false],
            [false, false, false, false, true],
        ],
        'z' => [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true],
        ],
        ' ' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        '.' => [
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        ',' => [
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, true, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        '!' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, true, true, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        '?' => [
            [false, false, false, false, false],
            [false, false, false, false, true],
            [true, false, true, false, true],
            [false, false, false, true, false],
            [false, false, false, false, false],
        ],
        '[' => [
            [false, false, false, false, false],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        ']' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, false],
        ],
        '(' | '{' | '<' => [
            [false, false, false, false, false],
            [false, true, true, true, false],
            [true, false, false, false, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        ')' | '}' | '>' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, true],
            [false, true, true, true, false],
            [false, false, false, false, false],
        ],
        '+' => [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],
        '-' => [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],
        ':' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, false, true, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
        '/' => [
            [true, false, false, false, false],
            [false, true, false, false, false],
            [false, false, true, false, false],
            [false, false, false, true, false],
            [false, false, false, false, true],
        ],
        '%' => [
            [true, false, false, false, true],
            [false, true, false, false, false],
            [false, false, true, false, false],
            [false, false, false, true, false],
            [true, false, false, false, true],
        ],
        _ => [
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
        ],
    }
}