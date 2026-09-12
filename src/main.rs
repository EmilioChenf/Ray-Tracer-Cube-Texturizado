mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod ray_intersect;
mod texture;

use camera::Camera;
use color::Color;
use cube::Cube;
use framebuffer::Framebuffer;
use light::Light;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, Vec3};
use ray_intersect::{Intersect, RayIntersect};
use std::f32::consts::PI;
use std::path::Path;
use texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND: Color = Color::new(9, 16, 32);
const FOV: f32 = PI / 3.0;
const ROTATION_SPEED: f32 = PI / 60.0;
const AMBIENT: f32 = 0.10;
const TEXTURE_PATH: &str = "assets/wall1.png";

fn shade(hit: &Intersect, light: &Light, texture: &Texture) -> Color {
    let light_direction = (light.position - hit.point).normalize();
    let diffuse = dot(&hit.normal, &light_direction).max(0.0) * light.intensity;
    texture
        .sample(hit.uv[0], hit.uv[1])
        .modulate(light.color)
        .scale(AMBIENT + diffuse)
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    objects: &[&dyn RayIntersect],
    light: &Light,
    texture: &Texture,
) -> Color {
    objects
        .iter()
        .filter_map(|object| object.ray_intersect(origin, direction))
        .min_by(|a, b| a.distance.total_cmp(&b.distance))
        .map_or(BACKGROUND, |hit| shade(&hit, light, texture))
}

fn render(
    framebuffer: &mut Framebuffer,
    cube: &Cube,
    camera: &Camera,
    light: &Light,
    texture: &Texture,
) {
    let aspect = framebuffer.width as f32 / framebuffer.height as f32;
    let scale = (FOV / 2.0).tan();
    let eye = camera.eye();
    let objects: [&dyn RayIntersect; 1] = [cube];

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x =
                (2.0 * (x as f32 + 0.5) / framebuffer.width as f32 - 1.0) * aspect * scale;
            let screen_y = (1.0 - 2.0 * (y as f32 + 0.5) / framebuffer.height as f32) * scale;
            let camera_ray = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let world_ray = camera.ray_direction(&camera_ray);
            let color = cast_ray(&eye, &world_ray, &objects, light, texture);
            framebuffer.point(x, y, color.to_hex());
        }
    }
}

fn scene() -> (Cube, Camera, Light) {
    let cube = Cube {
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(1.0, 1.0, 1.0),
    };
    let camera = Camera::new(Vec3::zeros(), 5.6, 0.64, 0.46);
    let light = Light {
        position: Vec3::new(-3.0, 5.0, 4.0),
        color: Color::new(255, 244, 232),
        intensity: 0.95,
    };
    (cube, camera, light)
}

fn render_to_file(path: &Path) -> std::io::Result<()> {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let (cube, camera, light) = scene();
    let texture = Texture::load(TEXTURE_PATH).map_err(std::io::Error::other)?;
    render(&mut framebuffer, &cube, &camera, &light, &texture);
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("bmp"))
    {
        framebuffer.save_bmp(path)
    } else {
        framebuffer.save_ppm(path)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--render") {
        let path = args.get(2).map_or(Path::new("cube.ppm"), Path::new);
        render_to_file(path).expect("no se pudo guardar la imagen PPM");
        println!("Imagen guardada en {}", path.display());
        return;
    }

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let (cube, mut camera, light) = scene();
    let mut window = Window::new(
        "Cubo texturizado - Flechas o WASD para orbitar - Esc para salir",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("no se pudo abrir la ventana");
    window.set_target_fps(60);
    let texture = Texture::load(TEXTURE_PATH).expect("no se pudo cargar assets/wall1.png");
    println!("Textura {}: {:?}", TEXTURE_PATH, texture.dimensions());

    let mut needs_render = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let controls = [
            ([Key::Left, Key::A], -ROTATION_SPEED, 0.0),
            ([Key::Right, Key::D], ROTATION_SPEED, 0.0),
            ([Key::Up, Key::W], 0.0, ROTATION_SPEED),
            ([Key::Down, Key::S], 0.0, -ROTATION_SPEED),
        ];
        for (keys, yaw, pitch) in controls {
            if keys.iter().any(|key| window.is_key_down(*key)) {
                camera.orbit(yaw, pitch);
                needs_render = true;
            }
        }
        if needs_render {
            render(&mut framebuffer, &cube, &camera, &light, &texture);
            needs_render = false;
        }
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("no se pudo actualizar la ventana");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambert_light_changes_with_surface_orientation() {
        let texture = Texture::load(TEXTURE_PATH).unwrap();
        let light = Light {
            position: Vec3::new(0.0, 0.0, 5.0),
            color: Color::new(255, 255, 255),
            intensity: 1.0,
        };
        let lit = Intersect {
            point: Vec3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            distance: 1.0,
            uv: [0.5, 0.5],
        };
        let away = Intersect {
            normal: Vec3::new(0.0, 0.0, -1.0),
            ..lit
        };
        assert!(shade(&lit, &light, &texture).to_hex() > shade(&away, &light, &texture).to_hex());
    }
}
