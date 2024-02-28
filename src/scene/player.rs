use super::camera::{Camera, CameraController};




pub struct Player {
    pub camera: Camera, 
    pub camera_controller:CameraController
}


impl Player {
    pub fn new() -> Self {
        let camera_controller = CameraController::new(2.0, 2.4);
        let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));

        Self {
            camera,
            camera_controller
        }
    }
}