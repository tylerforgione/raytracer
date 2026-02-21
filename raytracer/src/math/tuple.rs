// Tuple struct
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

// Tuple API
impl Tuple {
    // creates a new tuple
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {x, y, z, w}
    }

    // creates a point
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 1.0)
    }
    
    // creates a vector
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 0.0)
    }

    // return true if tuple is a point
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    // returns true if tuple is a vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    // returns magnitude of a tuple
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    // nromalizes a vector (sets its magnitude to 1)
    pub fn normalize(&self) -> Tuple {
        let a = self.magnitude();
        Tuple {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
            w: self.w / a
        }
    }

    // computes the dot product of two tuples
    pub fn dot(&self, b: Tuple) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }

    // computes cross product of two vectors
    pub fn cross(&self, b: Tuple) -> Tuple {
        Tuple::vector(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
}

use std::ops::Add;

// overload + for tuples
impl Add for Tuple {
    type Output = Tuple;

    fn add(self, b: Tuple) -> Tuple {
        Tuple {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w
        }
    }
}

use std::ops::Sub;

// overload - for tuples
impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, b: Tuple) -> Tuple {
        Tuple {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
            w: self.w - b.w
        }
    }
}

use std::ops::Neg;

// overload the negation operator (-x)
impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

use std::ops::Mul;

// overload * operator
impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, a: f64) -> Tuple {
        Tuple {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
            w: self.w * a
        }
    }
}

use std::ops::Div;

// overload / operator 
impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, a: f64) -> Tuple {
        Tuple {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
            w: self.w / a
        }
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }    

    fn approx_tuple_eq(a: Tuple, b: Tuple) -> bool {
        approx_eq(a.x, b.x)
            && approx_eq(a.y, b.y)
            && approx_eq(a.z, b.z)
            && approx_eq(a.w, b.w)
    }

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
    
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }
    
    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
    
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let result = a1 + a2;

        assert_eq!(result, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points_gives_a_vector() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        let result = p1 - p2;

        assert_eq!(result, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point_gives_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let result = p - v;

        assert_eq!(result, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        let result = zero - v;

        assert_eq!(result, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let t = Tuple::new(1.0, -2., 3., -4.);

        let result = -t;

        assert_eq!(result, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = a * 3.5;

        assert_eq!(result, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = a * 0.5;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = a / 2.0;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert!(approx_eq(v.magnitude(), 1.0));
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(approx_eq(v.magnitude(), 14.0_f64.sqrt()));
    }

    #[test]
    fn magnitude_of_vector_negative_1_2_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert!(approx_eq(v.magnitude(), 14.0_f64.sqrt()));
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        let result = v.normalize();

        assert!(approx_tuple_eq(
            result,
            Tuple::vector(1.0, 0.0, 0.0)
        ));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let result = v.normalize();

        assert!(approx_tuple_eq(
            result,
            Tuple::vector(0.26726, 0.53452, 0.80178)
        ));
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let norm = v.normalize();

        assert!(approx_eq(norm.magnitude(), 1.0));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let result = a.dot(b);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }
}