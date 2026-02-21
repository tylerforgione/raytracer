use std::ops::{Add, Sub, Mul};

// colour tuple (rgb)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    // initialize a new colour
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }
}

// overload + for colours
impl Add for Colour {
    type Output = Colour;

    fn add(self, b: Colour) -> Colour {
        Colour {
            r: self.r + b.r,
            g: self.g + b.g,
            b: self.b + b.b
        }
    }
}

// overload - for colours
impl Sub for Colour {
    type Output = Colour;

    fn sub(self, b: Colour) -> Colour {
        Colour {
            r: self.r - b.r,
            g: self.g - b.g,
            b: self.b - b.b
        }
    }
}

// overload * for colours (scalar)
impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, b: f64) -> Colour {
        Colour {
            r: self.r * b,
            g: self.g * b,
            b: self.b * b
        }
    }
}

// overload * for colours
impl Mul for Colour {
    type Output = Colour;

    fn mul(self, b: Colour) -> Colour {
        Colour {
            r: self.r * b.r,
            g: self.g * b.g,
            b: self.b * b.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }    

    fn approx_col_eq(a: Colour, b: Colour) -> bool {
        approx_eq(a.r, b.r)
            && approx_eq(a.g, b.g)
            && approx_eq(a.b, b.b)
    }

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        let result = c1 + c2;

        assert_eq!(result, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);

        let result = c1 - c2;

        assert!(approx_col_eq(result, Colour::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Colour::new(0.2, 0.3, 0.4);

        let result = c * 2.0;

        assert_eq!(result, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors_hadamard_product() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);

        let result = c1 * c2;

        assert!(approx_col_eq(result, Colour::new(0.9, 0.2, 0.04)));
    }

}
