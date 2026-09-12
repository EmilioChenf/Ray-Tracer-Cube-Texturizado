use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Exportación sencilla para validar/renderizar aun en un entorno sin ventana.
    pub fn save_ppm(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);
        writeln!(file, "P6\n{} {}\n255", self.width, self.height)?;
        for pixel in &self.buffer {
            file.write_all(&[(pixel >> 16) as u8, (pixel >> 8) as u8, *pixel as u8])?;
        }
        Ok(())
    }

    /// Guarda BMP de 24 bits, útil como imagen de entrega sin dependencias extra.
    pub fn save_bmp(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let row_size = (self.width * 3 + 3) & !3;
        let image_size = row_size * self.height;
        let file_size = 54 + image_size;
        let mut file = BufWriter::new(File::create(path)?);

        file.write_all(b"BM")?;
        file.write_all(&(file_size as u32).to_le_bytes())?;
        file.write_all(&[0; 4])?;
        file.write_all(&54_u32.to_le_bytes())?;
        file.write_all(&40_u32.to_le_bytes())?;
        file.write_all(&(self.width as i32).to_le_bytes())?;
        file.write_all(&(self.height as i32).to_le_bytes())?;
        file.write_all(&1_u16.to_le_bytes())?;
        file.write_all(&24_u16.to_le_bytes())?;
        file.write_all(&[0; 24])?;

        let padding = vec![0; row_size - self.width * 3];
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.buffer[y * self.width + x];
                file.write_all(&[pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8])?;
            }
            file.write_all(&padding)?;
        }
        Ok(())
    }
}
