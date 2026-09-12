use crate::color::Color;
use std::path::Path;

/// Imagen decodificada una sola vez y almacenada en el formato del renderer.
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Texture {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, image::ImageError> {
        let image = image::open(path)?.to_rgb8();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let pixels = image
            .pixels()
            .map(|pixel| Color::new(pixel[0], pixel[1], pixel[2]))
            .collect();
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// Nearest-neighbor con UV acotado. Se invierte V porque una imagen
    /// crece hacia abajo y el UV del cubo crece hacia arriba.
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let x = (u * (self.width - 1) as f32).round() as usize;
        let y = ((1.0 - v) * (self.height - 1) as f32).round() as usize;
        self.pixels[y * self.width + x]
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_project_texture_and_clamps_sampling() {
        let texture = Texture::load("assets/wall1.png").unwrap();
        assert_eq!(texture.dimensions(), (128, 128));
        assert_eq!(texture.sample(-1.0, 2.0), texture.sample(0.0, 1.0));
    }
}
