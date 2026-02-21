use raytracer::{canvas::Canvas, colour::Colour, math::tuple::Tuple, projectile::Projectile};

fn main() {
    let start: Tuple = Tuple::point(0.0, 1.0, 0.0);
    let velocity: Tuple = Tuple::vector(1.0, 1.0, 0.0).normalize() * 11.25;

    let mut p: Projectile = Projectile {
        position: start,
        velocity,
    };

    let gravity: Tuple = Tuple::vector(0.0, -0.1, 0.0);
    let wind: Tuple = Tuple::vector(-0.01, 0.0, 0.0);
    let e: raytracer::projectile::Environment =
        raytracer::projectile::Environment { gravity, wind };

    let mut canvas: Canvas = Canvas::new(900, 550);

    while p.position.y > 0.0 {
        let x = p.position.x as usize;
        let y = canvas.height - p.position.y as usize;

        if x < canvas.width && y < canvas.height {
            canvas.write_pixel(x, y, Colour::new(1.0, 0.5, 0.5));
        }

        p.position = p.position + p.velocity;
        p.velocity = p.velocity + e.gravity + e.wind;
    }

    std::fs::write("projectile.ppm", canvas.to_ppm()).expect("Unable to write file");
}
