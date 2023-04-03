// defines camera type

// defines 2d top down camera
pub struct Camera2D {
    pub eye: Point2<f32>,
    pub target: Point2<f32>,
    pub up: Vector2<f32>,
    pub speed: f32,
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub max_zoom: f32,
    pub min_zoom: f32,
}

// implements Camera2D
impl Camera2D {
    // creates a new camera
    pub fn new(eye: Point2<f32>, target: Point2<f32>, up: Vector2<f32>, speed: f32) -> Camera2D {
        Camera2D {
            eye,
            target,
            up,
            speed,
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
        }
    }

    pub fn zoom(&mut self, amount: f32) {
        let forward = self.target - self.eye;
        let forward_magnitude = forward.magnitude();

        self.target = self.eye + forward.normalize() * (forward_magnitude + amount);
    }

    // updates the camera
    pub fn update(&mut self) {
        use cgmath::InnerSpace;

        let forward = self.target - self.eye;
        let forward_normal = forward.normalize();
        let forward_magnitude = forward.magnitude();

        if self.move_forward && forward_magnitude > self.speed {
            self.eye += forward_normal * self.speed;
        }
        
        if self.move_backward {
            self.eye -= forward_normal * self.speed;
        }

        let right = forward_normal.cross(self.up);
        let forward = self.target - self.eye;
        let forward_magnitude = forward.magnitude();

        if self.move_right {
            self.target = self.eye + (forward + right * self.speed).normalize() * forward_magnitude;
        }

        
        if self.move_left {
            self.target = self.eye + (forward - right * self.speed).normalize() * forward_magnitude;
        }

        // implements zoom
        if self.zoom_in {
            // checks for max zoom
            if self.target.x - self.eye.x > self.max_zoom {
                return;
            }
            self.zoom(1.0);
            self.zoom_in = false;
        } else if self.zoom_out {
            // checks for min zoom
            if self.target.x - self.eye.x < self.min_zoom {
                return;
            }
            self.zoom(-1.0);
            self.zoom_out = false;
        }
    }

    // handles input
    pub fn handle_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == ElementState::Pressed;

                match input.virtual_keycode {
                    Some(VirtualKeyCode::W) => {
                        self.move_forward = pressed;
                        true
                    }
                    Some(VirtualKeyCode::S) => {
                        self.move_backward = pressed;
                        true
                    }
                    Some(VirtualKeyCode::A) => {
                        self.move_left = pressed;
                        true
                    }
                    Some(VirtualKeyCode::D) => {
                        self.move_right = pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

}