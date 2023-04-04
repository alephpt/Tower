#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

#[derive(Debug)]
pub struct Mouse {
    pub mouse_position: winit::dpi::PhysicalPosition<f64>,
    pub l_mouse_down: bool,
    pub m_mouse_down: bool,
    pub r_mouse_down: bool,
}

#[derive(Debug)]
pub struct Graphics {
    pub window: Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub mouse_state: Mouse,
}

impl Graphics {
    pub async fn new(window: Window) -> Self {
        // Initialize logger
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        // Initialize wgpu
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
        
        // get the window size
        let size = window.inner_size();

        // create the wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // create the wgpu surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // get the wgpu adapter
        let adapter = instance.enumerate_adapters(wgpu::Backends::all())
                              .filter(|adapter| {
                                  adapter.is_surface_supported(&surface)
                              })
                              .next().unwrap();
        
        // create the wgpu device and queue
        let (device, queue) = adapter.request_device(
                                          &wgpu::DeviceDescriptor {
                                              features: wgpu::Features::empty(),
                                              limits: if cfg!(target_arch = "wasm32") {
                                                wgpu::Limits::downlevel_webgl2_defaults()
                                                } else {
                                                    wgpu::Limits::default()
                                                },
                                              label: None
                                          }, None)
                                      .await.unwrap();

        // create the wgpu surface configuration
        let capabilities = surface.get_capabilities(&adapter);

        // get the surface format
        let surface_format = capabilities.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(capabilities.formats[0]);

        // create the wgpu surface configuration
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        // configure the surface
        surface.configure(&device, &config);
        
        // create the mouse state
        let mouse_state = Mouse {
            mouse_position: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            l_mouse_down: false,
            m_mouse_down: false,
            r_mouse_down: false,
        };

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            mouse_state
        }
    }

    pub fn new_window(event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new().build(event_loop).unwrap()
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }

    pub async fn run() -> Result<(), wgpu::SurfaceError> {
        let event_loop = EventLoop::new();
        let window = Graphics::new_window(&event_loop);
        let mut graphics = Graphics::new(window).await;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == graphics.window.id() => if !graphics.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        graphics.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        graphics.resize(**new_inner_size);
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        graphics.mouse_state.mouse_position = *position;
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        match (state, button) {
                            (ElementState::Pressed, MouseButton::Left) => {
                                graphics.mouse_state.l_mouse_down = true;
                            }
                            (ElementState::Released, MouseButton::Left) => {
                                graphics.mouse_state.l_mouse_down = false;
                            }
                            (ElementState::Pressed, MouseButton::Right) => {
                                graphics.mouse_state.m_mouse_down = true;
                            }
                            (ElementState::Released, MouseButton::Right) => {
                                graphics.mouse_state.m_mouse_down = false;
                            }
                            (ElementState::Pressed, MouseButton::Middle) => {
                                graphics.mouse_state.r_mouse_down = true;
                            }
                            (ElementState::Released, MouseButton::Middle) => {
                                graphics.mouse_state.r_mouse_down = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            },
            Event::RedrawRequested(_) => {
                graphics.update();
                match graphics.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => graphics.resize(graphics.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                graphics.window.request_redraw();
            },
            _ => {}
        });
    }
}
