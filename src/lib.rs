mod vector3;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::vector3::Vector3;

    #[test]
    fn copy() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let mut b = a;

        b.x += 1.0;

        assert!(a != b);
    }

    #[test]
    fn add() {
        let a = Vector3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let b = Vector3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(
            a + b,
            Vector3 {
                x: 5.0,
                y: 5.0,
                z: 5.0
            }
        );
    }

    #[test]
    fn sub() {
        let a = Vector3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let b = Vector3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(
            a - b,
            Vector3 {
                x: 1.0,
                y: -1.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn mul() {
        let a = Vector3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        assert_eq!(
            a * 5.0,
            Vector3 {
                x: 15.0,
                y: 10.0,
                z: 5.0
            }
        )
    }

    #[test]
    fn div() {
        let a = Vector3 {
            x: 5.0,
            y: 15.0,
            z: 30.0,
        };

        assert_eq!(
            a / 5.0,
            Vector3 {
                x: 1.0,
                y: 3.0,
                z: 6.0
            }
        )
    }

    #[test]
    fn magnitude() {
        let a = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 3.0,
        };

        assert_eq!(a.magnitude(), 3.0);
    }

    #[test]
    fn dot() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let b = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(a.dot(&b), 14.0);
    }

    #[test]
    fn cross() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let b = Vector3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        assert_eq!(
            a.cross(&b),
            Vector3 {
                x: -4.0,
                y: 8.0,
                z: -4.0
            }
        );
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
