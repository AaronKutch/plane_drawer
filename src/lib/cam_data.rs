
// use common::*;
use crate::coordinates::*;

pub const MAX_FOCAL_LEN_MULTIPLIER: D1 = D1::new_const("1000");
pub const MIN_FOCAL_LEN_MULTIPLIER: D1 = D1::new_const("20");

pub const MAX_CAMERA_SCALE: usize = 10;
pub const MIN_CAMERA_SCALE: usize = 1;

pub const LARGEST_ZOOM_EXPONENT: i8 = 9;
pub const SMALLEST_ZOOM_EXPONENT: i8 = -10;

#[derive(Debug, Clone, Copy)]



pub struct CamData {
    // if buffscale = 1, the buffer is the same pixel size as winsize, if 2, buffer is half the dimensions of winsize.
    // if drawing directly to the full sized window, a lower resolution buffer is not being used and all these buffer fields are unused
    buffscale: usize,
    winsize: (usize, usize),     // window dimensions in pixels
    buffsize: (usize, usize),    // buffer dimensions in pixels. = winsize / buffscale
    zoom: D1,             // equivilant to 1/(width of each tile in buffer pixels), gets bigger as cam zooms out
    pub zoom_exponent: i8,  // target zoom = 2^zoom_exponent, used to make sure our target zoom ends on an even number (1/4, 1/2, 1, 2, 4). Changing this value has no effect on actual zoom, only the target zoom level returned by target_zoom()
    view_dist: D1,          // how far the camera can see
    focal_len: D1,               // how far the camera ray origin is from the screen
    focal_len_multiplier: D1,   // exists because the focal length needs to change with zoom level (when screen widens, the focal len needs to lengthen proportionally or else parralax changes as the camera zooms in and out), instead of changing focal_len directly, we change this.
    pub cam_pos_rel: Pos,       // cam relative to current surface
    pub cam_pos_abs: Pos,       // cam relative to world origin
    pub current_surf_abs_pos: Pos, // current surface position in world
    pub cam_move_speed: D1,
}
impl CamData {
    pub fn new(winsize: (usize, usize), buffscale: usize) -> CamData {
        CamData {
            buffscale,
            zoom: D1::one(),
            zoom_exponent: 0,
            winsize,
            buffsize: (winsize.0 / buffscale, winsize.1 / buffscale),
            view_dist: D1::from(1000),
            focal_len: D1::from(100),
            focal_len_multiplier: D1::from(1000),
            cam_pos_rel: Pos::zero(),
            cam_pos_abs: Pos::zero(),
            current_surf_abs_pos: Pos::zero(),
            cam_move_speed: D1::from(10),
        }
    }
    pub fn update_resolution(&mut self) {
        self.buffsize.0 = self.winsize.0.div_ceil(self.buffscale);
        self.buffsize.1 = self.winsize.1.div_ceil(self.buffscale);
        self.focal_len = self.focal_len_multiplier * self.zoom;
    }
    /// if the camera is attatched to abs space, current_surf_abs_pos is just zero. If attatched to a surface, give the surface's absolute position
    pub fn update_cam_pos(&mut self, current_surf_abs_pos: Pos) {
        self.current_surf_abs_pos = current_surf_abs_pos;
        self.cam_pos_rel.angle.wrap(); // make angle wrap
        let angle = self.cam_pos_rel.angle + self.current_surf_abs_pos.angle;
        let rel_coords_rotated = self.cam_pos_rel.coords.rotate(angle);
        self.cam_pos_abs = Pos::new(rel_coords_rotated + self.current_surf_abs_pos.coords, angle);
    }

    pub fn change_resolution(&mut self, up: bool, winsize: (usize, usize)) {
        if up && self.buffscale() < MAX_CAMERA_SCALE {
            self.buffscale *= 2;
            self.zoom_exponent += 1;
        } else if !up && self.buffscale() > MIN_CAMERA_SCALE {
            self.buffscale /= 2;
            self.zoom_exponent -= 1;
        }
        self.update_winsize_and_buffsize(winsize);
    }
    /// does not actually change zoom level, just the exponent, which is used to keep track of our target zoom.
    /// does not change zoom level if it would go out of bounds
    pub fn change_zoom_exponent(&mut self, up: bool) {
        match up {
            false => {
                if self.zoom_exponent < LARGEST_ZOOM_EXPONENT {
                    self.zoom_exponent += 1;
                }
            }
            true => {
                if self.zoom_exponent > SMALLEST_ZOOM_EXPONENT {
                    self.zoom_exponent -= 1;
                }
            }
        }
    }
    /// returns target zoom level from zoom_exponent
    pub fn target_zoom(&self) -> D1 {
        D1::two().power(self.zoom_exponent as i32)
    }
    pub fn set_zoom(&mut self, zoom: D1) {
        self.zoom = zoom;
        self.update_resolution();
    }
    /// make sure both winsize dimensions are divisible by buffscale. Must not decrease size of winsize, only increase
    pub fn update_winsize_and_buffsize(&mut self, winsize: (usize, usize)) {
        self.winsize = (D2::from(winsize).ceil_to(D2::from(self.buffscale))).to_usize();
        self.update_resolution();
    }

    pub fn buffsize(&self) -> (usize, usize) {
        return self.buffsize
    }
    pub fn zoom(&self) -> D1 {
        return self.zoom
    }
    pub fn buffscale(&self) -> usize {
        return self.buffscale
    }
    pub fn winsize(&self) -> (usize, usize) {
        return self.winsize
    }
    pub fn focal_len(&self) -> D1 {
        return self.focal_len
    }
    /// only changes focal len multiplier if it does not go out of bounds because a negative fishere results in a crash
    pub fn change_focal_len_mutiplier(&mut self, add: D1) {
        if self.focal_len_multiplier + add >= MIN_FOCAL_LEN_MULTIPLIER && self.focal_len_multiplier + add <= MAX_FOCAL_LEN_MULTIPLIER {
            self.focal_len_multiplier += add;
            self.update_resolution();
        }
    }
    pub fn view_dist(&self) -> D1 {
        return self.view_dist
    }

    /// returns the zero position of the plane we want to draw to (relative to world origin).
    /// 
    /// a zero pos is the position of the draw plane relative to the center of the camera.
    /// A zero pos of {(0, 0, -50), angle: 0} would mean our draw plane is in the center of the screen 50 units below the camera and not rotated relative to the camera.
    /// Does not take camera zoom or fisheye into acount, is just the raw coords
    pub fn zero_pos(&self, abs_draw_plane_pos: Pos) -> Pos {
        return Pos::new(self.zero_coords(abs_draw_plane_pos), self.zero_angle(abs_draw_plane_pos));
    }

    pub fn zero_coords(&self, abs_target_pos: Pos) -> D3 {
        let cam_surf_abs_pos = self.current_surf_abs_pos;
        let cam_rel_pos = self.cam_pos_rel;
        let pos = abs_target_pos - cam_surf_abs_pos;

        let trans_coords = pos.coords.rotate(-(cam_surf_abs_pos.angle + cam_rel_pos.angle));

        return -cam_rel_pos.coords + trans_coords;
    }

    pub fn zero_angle(&self, abs_target_pos: Pos) -> Angle {
        let rel_cam_angle = self.cam_pos_rel.angle;
        let angle = abs_target_pos.angle - rel_cam_angle;

        return self.current_surf_abs_pos.angle - angle
    }
}