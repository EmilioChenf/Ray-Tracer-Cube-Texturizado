use nalgebra_glm::Vec3;
use std::f32::consts::FRAC_PI_2;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.05;

pub struct Camera {
    pub target: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(target: Vec3, radius: f32, yaw: f32, pitch: f32) -> Self {
        Self {
            target,
            radius,
            yaw,
            pitch: pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT),
        }
    }

    pub fn eye(&self) -> Vec3 {
        self.target
            + Vec3::new(
                self.radius * self.pitch.cos() * self.yaw.sin(),
                self.radius * self.pitch.sin(),
                self.radius * self.pitch.cos() * self.yaw.cos(),
            )
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    pub fn ray_direction(&self, camera_direction: &Vec3) -> Vec3 {
        let eye = self.eye();
        let forward = (self.target - eye).normalize();
        let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(&forward).normalize();
        (camera_direction.x * right + camera_direction.y * up - camera_direction.z * forward)
            .normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_preserves_radius_and_points_center_ray_at_target() {
        let mut camera = Camera::new(Vec3::zeros(), 5.0, 0.6, 0.4);
        camera.orbit(0.8, -0.2);
        assert!((camera.eye().magnitude() - 5.0).abs() < 1.0e-5);
        let expected = (camera.target - camera.eye()).normalize();
        assert!((camera.ray_direction(&Vec3::new(0.0, 0.0, -1.0)) - expected).magnitude() < 1.0e-5);
    }
}
