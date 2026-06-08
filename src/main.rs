use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop, window::Window};

// The coordinates for the so called seahorse valley, a nice valley to zoom into :)
const TARGET_X: f64 = -0.743643887037158704752191506114774;
const TARGET_Y: f64 = 0.131825904205311970493132056385139;

fn mandelbrot_iter(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let q = (cx - 0.25) * (cx - 0.25) + cy * cy;
    if q * (q + (cx - 0.25)) < 0.25 * cy * cy { return max_iter; }
    if (cx + 1.0) * (cx + 1.0) + cy * cy < 0.0625 { return max_iter; }

    let mut zx = 0.0; let mut zy = 0.0;
    let mut zx2 = 0.0; let mut zy2 = 0.0;
    let mut iteration = 0;

    while zx2 + zy2 <= 4.0 && iteration < max_iter {
        zy = 2.0 * zx * zy + cy;
        zx = zx2 - zy2 + cx;
        zx2 = zx * zx;
        zy2 = zy * zy;
        iteration += 1;
    }

    iteration
}

fn get_mandelbrot_color(iter: u32, max_iter: u32) -> Color {
    if iter == max_iter {
        Color::BLACK
    } else {
        let t: f64 = iter as f64 / 15.0; 
        let r: f64 = (t.sin() + 1.0) / 2.0;
        let g: f64 = ((t + 2.0).sin() + 1.0) / 2.0;
        let b: f64 = ((t + 4.0).sin() + 1.0) / 2.0;
        Color { r, g, b, a: 1.0 }
    }
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>, 
    frame_count: u32,
    zoom: f64
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            frame_count: 0,
            zoom: 1.0
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Mandelbrot Visualisation")
                .with_inner_size(LogicalSize::new(1280, 720));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let window_size = window.clone().inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            let pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

            self.window = Some(window);
            self.pixels = Some(pixels);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event { 
            WindowEvent::CloseRequested => {event_loop.exit();} 
            WindowEvent::Resized(new_size) => {
                if let Some(pixels) = &mut self.pixels { if pixels.resize_surface(new_size.width, new_size.height).is_err() { event_loop.exit(); } }
            }
            WindowEvent::RedrawRequested => {
                    if let (Some(pixels), Some(window)) = (&mut self.pixels, &self.window) {
                    let frame = pixels.frame_mut();
                    self.frame_count = self.frame_count.wrapping_add(1);

                    let h: usize = 720;
                    let w: usize = 1280;

                    self.zoom *= 1.01;
                    let current_max_iter = (200.0 + self.zoom.ln() * 30.0).min(1000.0) as u32;

                    let y_range = 2.5 / self.zoom;
                    let aspect_ratio = w as f64 / h as f64;
                    let x_range = y_range * aspect_ratio;
                    let x_min = TARGET_X - (x_range / 2.0);
                    let y_min = TARGET_Y - (y_range / 2.0);
                    
                    if y_range < 1e-14 { self.zoom = 1.0; }

                    frame.par_chunks_exact_mut(4).enumerate().for_each(|(index, pixel)| {
                        let col = index % w;
                        let row = index / w;

                        let cx = x_min + (col as f64 / w as f64) * x_range;
                        let cy = y_min + (row as f64 / h as f64) * y_range;
                        let iter = mandelbrot_iter(cx, cy, current_max_iter);
                        let color = get_mandelbrot_color(iter, current_max_iter);

                        if color == Color::BLACK { pixel[0] = 0; pixel[1] = 0;  pixel[2] = 0;  pixel[3] = 255;  }
                        else { 
                            pixel[0] = (color.r * 255.0) as u8; 
                            pixel[1] = (color.g * 255.0) as u8; 
                            pixel[2] = (color.b * 255.0) as u8; 
                            pixel[3] = 255; 
                        }
                    });

                    if pixels.render().is_err() { return; }
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
