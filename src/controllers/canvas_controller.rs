use std::sync::Arc;

use druid::{Color, Data, Lens};
use image::{ImageBuffer, Rgba};

#[derive(Clone, Data, Lens)]
pub struct CanvasController {
    pub canvas_width: usize,
    pub canvas_height: usize,
    pub pixel_size: f64,
    pub pixel_data: Arc<Vec<Vec<Color>>>,
    pub frames_data: Arc<Vec<Vec<Vec<Color>>>>,
    pub curr_frame: usize,
    pub brush_color: Color,
    pub brush_size: usize,
}

impl CanvasController {
    pub fn new() -> Self {
        let def_pixel_data = vec![vec![Color::WHITE; 16]; 16];
        let def_frames_data = vec![def_pixel_data.clone()];

        CanvasController {
            canvas_width: 16,
            canvas_height: 16,
            pixel_size: 25.0,
            pixel_data: Arc::new(def_pixel_data),
            frames_data: Arc::new(def_frames_data),
            curr_frame: 0,
            brush_color: Color::BLACK,
            brush_size: 1,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        if x < self.canvas_width && y < self.canvas_height {
            let mut drew_pix = (*self.pixel_data).clone();
            for i in 0..self.brush_size {
                for j in 0..self.brush_size {
                    let px = x + i;
                    let py = y + j;
                    if px < self.canvas_width && py < self.canvas_height {
                        drew_pix[py][px] = self.brush_color.clone();
                    }
                }
            }
            self.pixel_data = Arc::new(drew_pix);

            let mut created_frames = (*self.frames_data).clone();
            created_frames[self.curr_frame] = (*self.pixel_data).clone();
            self.frames_data = Arc::new(created_frames);
        }
    }

    pub fn save_img(&self, path: &str) {
        let mut img = ImageBuffer::new(self.canvas_width as u32, self.canvas_height as u32);
        for (y, row) in self.pixel_data.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                let (r, g, b, a) = color.as_rgba8();
                img.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
            }
        }
        img.save(path).expect("Failed to save PNG");
    }

    pub fn zoom(&mut self, a: f64) {
        self.pixel_size += a;
    }
}
