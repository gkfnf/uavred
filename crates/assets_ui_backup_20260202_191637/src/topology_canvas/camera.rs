//! Camera/Viewport management for topology canvas

/// Camera state managing viewport transform
#[derive(Clone, Debug)]
pub struct Camera {
    /// Zoom scale (1.0 = 100%)
    pub scale: f32,
    /// Viewport offset in virtual coordinates (top-left corner)
    pub offset_x: f32,
    pub offset_y: f32,
    /// Viewport size in screen pixels
    pub viewport_width: f32,
    pub viewport_height: f32,
}

/// Bounds in virtual coordinates
#[derive(Clone, Debug, Default)]
pub struct VirtualBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl VirtualBounds {
    pub fn from_points(points: &[(f32, f32)], padding: f32) -> Self {
        if points.is_empty() {
            return Self::default();
        }

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for (x, y) in points {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        Self {
            min_x: min_x - padding,
            max_x: max_x + padding,
            min_y: min_y - padding,
            max_y: max_y + padding,
        }
    }

    pub fn width(&self) -> f32 {
        (self.max_x - self.min_x).max(1.0)
    }

    pub fn height(&self) -> f32 {
        (self.max_y - self.min_y).max(1.0)
    }

    pub fn center(&self) -> (f32, f32) {
        ((self.min_x + self.max_x) / 2.0, (self.min_y + self.max_y) / 2.0)
    }

    pub fn is_valid(&self) -> bool {
        self.width() > 0.0 && self.height() > 0.0
    }
}

impl Camera {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            viewport_width,
            viewport_height,
        }
    }

    /// Transform virtual coordinate to screen coordinate
    /// screen_x = (virtual_x - offset_x) * scale
    pub fn virtual_to_screen(&self, vx: f32, vy: f32) -> (f32, f32) {
        let sx = (vx - self.offset_x) * self.scale;
        let sy = (vy - self.offset_y) * self.scale;
        (sx, sy)
    }

    /// Transform screen coordinate to virtual coordinate
    /// virtual_x = screen_x / scale + offset_x
    pub fn screen_to_virtual(&self, sx: f32, sy: f32) -> (f32, f32) {
        let vx = sx / self.scale + self.offset_x;
        let vy = sy / self.scale + self.offset_y;
        (vx, vy)
    }

    /// Zoom at a specific screen point, keeping that point stable
    pub fn zoom_at(&mut self, screen_x: f32, screen_y: f32, new_scale: f32) {
        // Calculate the virtual point under the mouse before zoom
        let vx = screen_x / self.scale + self.offset_x;
        let vy = screen_y / self.scale + self.offset_y;

        // Apply new scale
        self.scale = new_scale.clamp(MIN_ZOOM, MAX_ZOOM);

        // Adjust offset so the same virtual point remains under the mouse
        self.offset_x = vx - screen_x / self.scale;
        self.offset_y = vy - screen_y / self.scale;
    }

    /// Pan by screen delta (for mouse drag)
    /// When dragging, if we move mouse by (dx, dy), the offset changes in opposite direction
    pub fn pan_by_screen(&mut self, screen_dx: f32, screen_dy: f32) {
        // screen_dx = virtual_dx * scale
        // virtual_dx = screen_dx / scale
        // To pan right (positive screen_dx), we need to increase offset_x
        self.offset_x += screen_dx / self.scale;
        self.offset_y += screen_dy / self.scale;
    }

    /// Pan by virtual delta (for trackpad)
    pub fn pan_by_virtual(&mut self, virtual_dx: f32, virtual_dy: f32) {
        self.offset_x += virtual_dx;
        self.offset_y += virtual_dy;
    }

    /// Fit the camera to show the given bounds with optional padding
    pub fn fit_to_bounds(&mut self, bounds: &VirtualBounds, padding_factor: f32) {
        if !bounds.is_valid() {
            return;
        }

        let (center_x, center_y) = bounds.center();
        let content_width = bounds.width();
        let content_height = bounds.height();

        // Calculate scale to fit content with padding
        let scale_x = self.viewport_width / (content_width * padding_factor);
        let scale_y = self.viewport_height / (content_height * padding_factor);

        // Use the smaller scale to ensure content fits in both dimensions
        self.scale = scale_x.min(scale_y).clamp(MIN_ZOOM, MAX_ZOOM);

        // Center the content: offset = center - viewport_size/(2*scale)
        self.offset_x = center_x - self.viewport_width / (2.0 * self.scale);
        self.offset_y = center_y - self.viewport_height / (2.0 * self.scale);
    }

    /// Get the current center point in virtual coordinates
    pub fn get_center(&self) -> (f32, f32) {
        self.screen_to_virtual(self.viewport_width / 2.0, self.viewport_height / 2.0)
    }
}

/// Zoom constraints
pub const MIN_ZOOM: f32 = 0.1;
pub const MAX_ZOOM: f32 = 5.0;
pub const ZOOM_STEP_FACTOR: f32 = 1.15;

/// Calculate new zoom scale from scroll delta
pub fn calculate_zoom_from_scroll(delta_y: f32, current_scale: f32) -> f32 {
    // delta_y < 0: scroll up/zoom in (on Mac with natural scrolling)
    // delta_y > 0: scroll down/zoom out
    let direction = if delta_y < 0.0 { 1.0 } else { -1.0 };
    let factor = ZOOM_STEP_FACTOR.powf(direction);
    (current_scale * factor).clamp(MIN_ZOOM, MAX_ZOOM)
}
