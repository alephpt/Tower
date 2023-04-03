// Window and Render pipeline implementing surface type

use crate::types::surface::Surface;
use crate::types::grid::Grid;
use crate::types::cube::Cube;
use crate::types::line::Line;
use crate::types::camera::Camera;

pub struct Window {
    pub surface: Surface,
    pub display: glium::Display,
    pub event_loop: glutin::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new(surface: Surface) -> Window {
        let mut builder = glutin::window::WindowBuilder::new();
        builder = builder.with_title(&surface.title);
        builder = builder.with_inner_size(glutin::dpi::LogicalSize::new(
            surface.width as f64,
            surface.height as f64,
        ));
        builder = builder.with_resizable(surface.resizable);
        builder = builder.with_fullscreen(if surface.fullscreen {
            Some(glutin::window::Fullscreen::Borderless(None))
        } else {
            None
        });

        let context = glutin::ContextBuilder::new();
        let context = context.with_vsync(surface.vsync);

        let event_loop = glutin::event_loop::EventLoop::new();
        let display = glium::Display::new(builder, context, &event_loop).unwrap();

        Window {
            surface,
            display,
            event_loop,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.surface.get_dimensions()
    }

    pub fn get_half_dimensions(&self) -> (f32, f32) {
        self.surface.get_half_dimensions()
    }

    pub fn get_display(&self) -> &glium::Display {
        &self.display
    }

    pub fn get_event_loop(&self) -> &glutin::event_loop::EventLoop<()> {
        &self.event_loop
    }

    pub fn get_clear_color(&self) -> (f32, f32, f32, f32) {
        self.surface.clear_color
    }

    pub fn add_cube(&mut self, width: f32, height: f32, depth: f32) -> Cube {
        Cube::new(width, height, depth, &self.display)
    }

    pub fn add_line(&mut self, width: f32, height: f32, depth: f32) -> Line {
        Line::new(width, height, depth, &self.display)
    }

    pub fn create_grid(&mut self, width: u32, height: u32, depth: u32) -> Grid {
        Grid::new(width, height, depth, &self.display)
    }

    pub fn create_camera(&mut self) -> Camera {
        Camera::new(&self.display)
    }

    pub fn create_light(&mut self) -> Light {
        Light::new(&self.display)
    }

    pub fn create_text(&mut self, text: &str) -> Text {
        Text::new(text, &self.display)
    }

    pub fn create_sprite(&mut self, width: f32, height: f32) -> Sprite {
        Sprite::new(width, height, &self.display)
    }

    pub fn draw_towers(&mut self, towers: &Vec<Tower>) {
        for tower in towers {
            tower.draw(&self.display);
        }
    }

    pub fn draw_line(&mut self, start: &Point3<f32>, end: &Point3<f32>) {
        let mut line = self.add_line(0.1, 0.1, 0.1);
        line.set_color(1.0, 1.0, 1.0);
        line.set_local_translation(Translation3::new(start.x, start.y, start.z));
        line.set_local_rotation(UnitQuaternion::face_towards(
            &Vector3::new(end.x, end.y, end.z),
            &Vector3::new(0.0, 0.0, 1.0),
        ));
        line.draw(&self.display);
    }

    pub fn draw_grid(&mut self, grid: &Grid) {
        grid.draw(&self.display);
    }
}