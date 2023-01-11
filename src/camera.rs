pub enum Movement {
    Forward,
    BackWard,
    Left,
    Right,
}

pub struct Camera {
    position: glam::Vec3,
    front: glam::Vec3,
    up: glam::Vec3,
    right: glam::Vec3,
    world_up: glam::Vec3,
    // Eular Angles
    yaw: f32,
    pitch: f32,
    // Camera options
    movement_speed: f32,
    senitivity: f32,
    zoom: f32,
}

impl Camera {
    // pub fn new(position: glam::Vec3, up: glam::Vec3, pitch: f32, yaw: f32) -> Self {

    // }

    pub fn get_viewmatrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn move_position(&mut self, direction: Movement, delta_time: f32) {
        let velocity = delta_time * self.movement_speed;
        match direction {
            Movement::Forward => self.position += self.front * velocity,
            Movement::BackWard => self.position -= self.front * velocity,
            Movement::Left => self.position -= self.front.cross(self.up).normalize() * velocity,
            Movement::Right => self.position += self.front.cross(self.up).normalize() * velocity,
        }
    }

    pub fn move_view(&mut self, x_offset: f32, y_offset: f32) {
        let x = x_offset * self.senitivity;
        let y = y_offset * self.senitivity;

        self.yaw += x;
        self.pitch += y;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_camera_vectors();
    }

    pub fn change_zoom(&mut self, offset: f32) {
        self.zoom -= offset;
        self.zoom = self.zoom.clamp(1.0, 90.0);
    }

    pub fn fov(&self) -> f32 {
        self.zoom
    }

    pub fn position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn front(&self) -> glam::Vec3 {
        self.front
    }

    fn update_camera_vectors(&mut self) {
        self.front = glam::Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut cam = Camera {
            position: glam::vec3(0.0, 0.0, 0.0),
            front: glam::vec3(0.0, 0.0, -1.0),
            up: glam::Vec3::default(),
            right: glam::Vec3::default(),
            world_up: glam::Vec3::Y,
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 5.0,
            senitivity: 0.1,
            zoom: 45.0,
        };

        cam.update_camera_vectors();
        cam
    }
}
