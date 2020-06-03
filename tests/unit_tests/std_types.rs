use float_eq::{float_eq, FloatDiff, FloatEqAllDebug, FloatEqDebug};
use std::rc::Rc;
use std::sync::Arc;

mod rc {
    use super::*;

    #[test]
    fn float_diff() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Rc::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Rc::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Rc::new([1.0f32, 2.0]);
        let b = Rc::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}
mod arc {
    use super::*;

    #[test]
    fn float_diff() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);
        assert_eq!(a.abs_diff(&b), [0.5, 0.25]);

        let c = Arc::new([1.000_000_1f32, 2.000_000_5]);
        assert_eq!(a.ulps_diff(&c), Some([1, 2]));
    }

    #[test]
    fn float_eq() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);
        assert!(float_eq!(a, b, abs <= [0.5, 0.25]));
        assert!(float_eq!(a, b, abs_all <= 0.5));

        let c = Arc::new([1.000_000_1f32, 2.000_000_5]);
        assert!(float_eq!(a, c, rel <= [f32::EPSILON, 2.0 * f32::EPSILON]));
        assert!(float_eq!(a, c, rel_all <= 2.0 * f32::EPSILON));

        assert!(float_eq!(a, c, ulps <= [1, 2]));
        assert!(float_eq!(a, c, ulps_all <= 2));
    }

    #[test]
    fn float_eq_debug() {
        let a = Arc::new([1.0f32, 2.0]);
        let b = Arc::new([1.5f32, 2.25]);

        assert_eq!(a.debug_abs_epsilon(&b, &[0.1, 0.2]), [0.1, 0.2]);
        assert_eq!(a.debug_abs_all_epsilon(&b, &0.2), [0.2, 0.2]);

        assert_eq!(a.debug_rel_epsilon(&b, &[0.1, 0.5]), [0.15, 1.125]);
        assert_eq!(a.debug_rel_all_epsilon(&b, &0.5), [0.75, 1.125]);

        assert_eq!(a.debug_ulps_epsilon(&b, &[1, 2]), [1, 2]);
        assert_eq!(a.debug_ulps_all_epsilon(&b, &2), [2, 2]);
    }
}
