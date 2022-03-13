mod vector3;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::vector3::Vector3;

    #[test]
    fn copy() {
        let a = Vector3::from_i32(1, 2, 3);

        let mut b = a;

        b.x += 1.0;

        assert!(a != b);
    }

    #[test]
    fn add() {
        let a = Vector3::from_i32(3, 2, 1);
        let b = Vector3::from_i32(2, 3, 4);

        assert_eq!(a + b, Vector3::from_i32(5, 5, 5));
    }

    #[test]
    fn sub() {
        let a = Vector3::from_i32(3, 2, 1);
        let b = Vector3::from_i32(2, 3, 4);
        assert_eq!(a - b, Vector3::from_i32(1, -1, -3));
    }

    #[test]
    fn mul() {
        let a = Vector3::from_i32(3, 2, 1);

        assert_eq!(a * 5.0, Vector3::from_i32(15, 10, 5))
    }

    #[test]
    fn div() {
        let a = Vector3::from_i32(5, 15, 30);
        assert_eq!(a / 5.0, Vector3::from_i32(1, 3, 6))
    }

    #[test]
    fn magnitude() {
        let a = Vector3::from_i32(0, 0, 3);

        assert_eq!(a.magnitude(), 3.0);
    }

    #[test]
    fn dot() {
        let a = Vector3::from_i32(1, 2, 3);
        let b = Vector3::from_i32(1, 2, 3);

        assert_eq!(a.dot(&b), 14.0);
    }

    #[test]
    fn cross() {
        let a = Vector3::from_i32(1, 2, 3);
        let b = Vector3::from_i32(3, 2, 1);

        assert_eq!(a.cross(&b), Vector3::from_i32(-4, 8, -4));
    }

    #[test]
    fn angle() {
        let a = Vector3::from_i32(1, 0, 0);
        let b = Vector3::from_i32(0, 0, 1);

        assert_eq!(a.angle(b) * (180.0 / PI), 90.0);
    }

    #[test]
    fn angle_deg() {
        let a = Vector3::from_i32(1, 0, 0);
        let b = Vector3::from_i32(0, 0, 1);

        assert_eq!(a.angle_deg(b), 90.0);
    }

    #[test]
    fn normalize() {
        let a = Vector3::new(0.5, 0.0, 0.0);

        assert_eq!(a.normalize(), Vector3::from_i32(1, 0, 0));
    }
}
