# Ray Tracer Cube Texturizado

Ray tracer académico en Rust que renderiza un cubo AABB con textura de piedra,
UV mapping para sus seis caras, iluminación Lambertiana difusa y cámara orbital.
No contiene reflexión, refracción ni componente especular.

La textura se carga una vez desde `assets/wall1.png`, tomada de la rama
`11-RC-05-MAZE-TEXTURES` del proyecto de referencia del profesor.

## Ejecutar

```powershell
cargo run
```

Controles: flechas o `WASD` para orbitar; `Esc` para salir.

Para generar una imagen sin abrir una ventana:

```powershell
cargo run -- --render textured-cube.bmp
```
