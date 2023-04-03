// openGL surface type
//
pub struct Surface {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
    pub resizable: bool,
    pub clear_color: (f32, f32, f32, f32),
}

impl Surface {
    pub fn new() -> Surface {
        Surface {
            width: 800,
            height: 600,
            title: String::from("Tower"),
            fullscreen: false,
            vsync: true,
            resizable: true,
            clear_color: (0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_half_dimensions(&self) -> (f32, f32) {
        (self.width as f32 / 2.0, self.height as f32 / 2.0)
    }

    pub fn init(&self) {
        let mut builder = glutin::window::WindowBuilder::new();
        builder = builder.with_title(&self.title);
        builder = builder.with_inner_size(glutin::dpi::LogicalSize::new(
            self.width as f64,
            self.height as f64,
        ));
        builder = builder.with_resizable(self.resizable);
        builder = builder.with_fullscreen(if self.fullscreen {
            Some(glutin::window::Fullscreen::Borderless(None))
        } else {
            None
        });

        let context = glutin::ContextBuilder::new();
        let context = context.with_vsync(self.vsync);

        let event_loop = glutin::event_loop::EventLoop::new();
        let display = glium::Display::new(builder, context, &event_loop).unwrap();

        let mut target = display.draw();
        target.clear_color_and_depth(self.clear_color, 1.0);
        target.finish().unwrap();
    }

    pub fn resize(&self) {
        unimplemented!("Surface resize not implemented");
    }

    pub fn update(&self) {
        unimplemented!("Surface update not implemented");
    }
}