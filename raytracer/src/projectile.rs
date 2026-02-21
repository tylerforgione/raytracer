use crate::math::tuple::Tuple;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple
}

pub fn tick(e: &Environment, p: &Projectile) -> Projectile {
    Projectile {
        position: p.position + p.velocity,
        velocity: p.velocity + e.gravity + e.wind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_moves() {
        let p = Projectile {
            position: Tuple::point(0.0, 1.0, 0.0),
            velocity: Tuple::vector(1.0, 1.0, 0.0),
        };

        let env = Environment {
            gravity: Tuple::vector(0.0, -0.1, 0.0),
            wind: Tuple::vector(-0.01, 0.0, 0.0),
        };

        let p2 = tick(&env, &p);

        assert_eq!(
            p2.position,
            Tuple::point(1.0, 2.0, 0.0)
        );
    }
}
