#![allow(clippy::float_cmp)]

use core::f32;
use float_eq::{
    assert_float_eq, assert_float_ne, float_eq, float_ne, ComplexUlps32, FloatDiff, FloatEq,
    FloatEqAll, FloatEqAllDebug, FloatEqDebug,
};
use num_complex::Complex32;

#[test]
fn complex_ulps() {
    let a = ComplexUlps32::new(1, 2);
    assert_eq!(a, a);
    let mut b = a.clone();
    b.im = 3;
    assert_ne!(a, b);
}

#[test]
fn float_diff() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_1, -2.000_000_5);
    let c = Complex32::new(1.000_000_1, 2.000_000_5);

    assert_eq!(
        a.abs_diff(&b),
        Complex32::new(0.000_000_119_209_29, 0.000_000_476_837_16)
    );
    assert_eq!(
        a.abs_diff(&c),
        Complex32::new(0.000_000_119_209_29, 4.0000005)
    );

    assert_eq!(a.ulps_diff(&b), Some(ComplexUlps32::new(1, 2)));
    assert_eq!(a.ulps_diff(&c), None);
}

#[test]
fn float_eq_methods() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert!(a.eq_abs(
        &b,
        &Complex32 {
            re: (2. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    ));
    assert!(a.ne_abs(
        &b,
        &Complex32 {
            re: (1. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    ));
    assert!(a.ne_abs(
        &b,
        &Complex32 {
            re: (2. * f32::EPSILON),
            im: (7. * f32::EPSILON)
        }
    ));

    assert!(a.eq_rel(
        &b,
        &Complex32 {
            re: (2. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    ));
    assert!(a.ne_rel(
        &b,
        &Complex32 {
            re: (1. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    ));
    assert!(a.ne_rel(
        &b,
        &Complex32 {
            re: (2. * f32::EPSILON),
            im: (3. * f32::EPSILON)
        }
    ));

    assert!(a.eq_ulps(&b, &ComplexUlps32 { re: 2, im: 4 }));
    assert!(a.ne_ulps(&b, &ComplexUlps32 { re: 1, im: 4 }));
    assert!(a.ne_ulps(&b, &ComplexUlps32 { re: 2, im: 3 }));
}

#[test]
fn float_eq_all_methods() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert!(a.eq_abs_all(&b, &(8. * f32::EPSILON)));
    assert!(a.ne_abs_all(&b, &(7. * f32::EPSILON)));

    assert!(a.eq_rel_all(&b, &(4. * f32::EPSILON)));
    assert!(a.ne_rel_all(&b, &(3. * f32::EPSILON)));

    assert!(a.eq_ulps_all(&b, &4));
    assert!(a.ne_ulps_all(&b, &2));
}

#[test]
fn float_eq_macros() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert!(float_eq!(
        a,
        b,
        abs <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    ));
    assert!(float_ne!(
        a,
        b,
        abs <= Complex32 {
            re: (1. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    ));
    assert!(float_ne!(
        a,
        b,
        abs <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (7. * f32::EPSILON)
        }
    ));

    assert!(float_eq!(
        a,
        b,
        rel <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= Complex32 {
            re: (1. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    ));
    assert!(float_ne!(
        a,
        b,
        rel <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (3. * f32::EPSILON)
        }
    ));

    assert!(float_eq!(a, b, ulps <= ComplexUlps32 { re: 2, im: 4 }));
    assert!(float_ne!(a, b, ulps <= ComplexUlps32 { re: 1, im: 4 }));
    assert!(float_ne!(a, b, ulps <= ComplexUlps32 { re: 2, im: 3 }));
}

#[test]
fn float_eq_all_macros() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert!(float_eq!(a, b, abs_all <= (8. * f32::EPSILON)));
    assert!(float_ne!(a, b, abs_all <= (7. * f32::EPSILON)));

    assert!(float_eq!(a, b, rel_all <= (4. * f32::EPSILON)));
    assert!(float_ne!(a, b, rel_all <= (3. * f32::EPSILON)));

    assert!(float_eq!(a, b, ulps_all <= 4));
    assert!(float_ne!(a, b, ulps_all <= 3));
}

#[test]
fn float_eq_debug() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    let eps = Complex32 {
        re: (2. * f32::EPSILON),
        im: (8. * f32::EPSILON),
    };
    assert_eq!(
        a.debug_abs_epsilon(&b, &eps),
        Complex32 {
            re: a.re.debug_abs_epsilon(&b.re, &eps.re),
            im: a.im.debug_abs_epsilon(&b.im, &eps.im),
        }
    );

    let eps = Complex32 {
        re: (2. * f32::EPSILON),
        im: (4. * f32::EPSILON),
    };
    assert_eq!(
        a.debug_rel_epsilon(&b, &eps),
        Complex32 {
            re: a.re.debug_rel_epsilon(&b.re, &eps.re),
            im: a.im.debug_rel_epsilon(&b.im, &eps.im),
        }
    );

    let eps = ComplexUlps32 { re: 2, im: 4 };
    assert_eq!(
        a.debug_ulps_epsilon(&b, &eps),
        ComplexUlps32 {
            re: a.re.debug_ulps_epsilon(&b.re, &eps.re),
            im: a.im.debug_ulps_epsilon(&b.im, &eps.im),
        }
    );
}

#[test]
fn float_eq_all_debug() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    let eps = 8. * f32::EPSILON;
    assert_eq!(
        a.debug_abs_all_epsilon(&b, &eps),
        Complex32 {
            re: a.re.debug_abs_all_epsilon(&b.re, &eps),
            im: a.im.debug_abs_all_epsilon(&b.im, &eps),
        }
    );

    let eps = 4. * f32::EPSILON;
    assert_eq!(
        a.debug_rel_all_epsilon(&b, &eps),
        Complex32 {
            re: a.re.debug_rel_all_epsilon(&b.re, &eps),
            im: a.im.debug_rel_all_epsilon(&b.im, &eps),
        }
    );

    let eps = 4;
    assert_eq!(
        a.debug_ulps_all_epsilon(&b, &eps),
        ComplexUlps32 {
            re: a.re.debug_ulps_all_epsilon(&b.re, &eps),
            im: a.im.debug_ulps_all_epsilon(&b.im, &eps),
        }
    );
}

#[test]
fn assert_float_eq() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert_float_eq!(
        a,
        b,
        abs <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    );
    assert_float_ne!(
        a,
        b,
        abs <= Complex32 {
            re: (1. * f32::EPSILON),
            im: (8. * f32::EPSILON)
        }
    );
    assert_float_ne!(
        a,
        b,
        abs <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (7. * f32::EPSILON)
        }
    );

    assert_float_eq!(
        a,
        b,
        rel <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    );
    assert_float_ne!(
        a,
        b,
        rel <= Complex32 {
            re: (1. * f32::EPSILON),
            im: (4. * f32::EPSILON)
        }
    );
    assert_float_ne!(
        a,
        b,
        rel <= Complex32 {
            re: (2. * f32::EPSILON),
            im: (3. * f32::EPSILON)
        }
    );

    assert_float_eq!(a, b, ulps <= ComplexUlps32 { re: 2, im: 4 });
    assert_float_ne!(a, b, ulps <= ComplexUlps32 { re: 1, im: 4 });
    assert_float_ne!(a, b, ulps <= ComplexUlps32 { re: 2, im: 3 });
}

#[test]
fn assert_float_eq_all() {
    let a = Complex32::new(1., -2.);
    let b = Complex32::new(1.000_000_2, -2.000_001);

    assert_float_eq!(a, b, abs_all <= 8. * f32::EPSILON);
    assert_float_ne!(a, b, abs_all <= 7. * f32::EPSILON);

    assert_float_eq!(a, b, rel_all <= 4. * f32::EPSILON);
    assert_float_ne!(a, b, rel_all <= 2. * f32::EPSILON);

    assert_float_eq!(a, b, ulps_all <= 4);
    assert_float_ne!(a, b, ulps_all <= 3);
}

#[test]
#[should_panic(expected = r#"`float_eq!(left, right, abs <= ε, rel <= ε, ulps <= ε)`
        left: `Complex { re: 1.0, im: 2.0 }`,
       right: `Complex { re: 3.0, im: 5.0 }`,
    abs_diff: `Complex { re: 2.0, im: 3.0 }`,
   ulps_diff: `Some(ComplexUlps { re: 12582912, im: 10485760 })`,
     [abs] ε: `Complex { re: 0.1, im: 0.25 }`,
     [rel] ε: `Complex { re: 0.3, im: 1.25 }`,
    [ulps] ε: `ComplexUlps { re: 1, im: 2 }`"#)]
fn assert_fail_message() {
    assert_float_eq!(
        Complex32::new(1., 2.),
        Complex32::new(3., 5.),
        abs <= Complex32::new(0.1, 0.25),
        rel <= Complex32::new(0.1, 0.25),
        ulps <= ComplexUlps32::new(1, 2)
    );
}

#[test]
#[should_panic(
    expected = r#"`float_eq!(left, right, abs_all <= ε, rel_all <= ε, ulps_all <= ε)`
        left: `Complex { re: 1.0, im: 2.0 }`,
       right: `Complex { re: 3.0, im: 5.0 }`,
    abs_diff: `Complex { re: 2.0, im: 3.0 }`,
   ulps_diff: `Some(ComplexUlps { re: 12582912, im: 10485760 })`,
 [abs_all] ε: `Complex { re: 0.25, im: 0.25 }`,
 [rel_all] ε: `Complex { re: 0.75, im: 1.25 }`,
[ulps_all] ε: `ComplexUlps { re: 3, im: 3 }`"#
)]
fn assert_fail_all_message() {
    assert_float_eq!(
        Complex32::new(1., 2.),
        Complex32::new(3., 5.),
        abs_all <= 0.25,
        rel_all <= 0.25,
        ulps_all <= 3
    );
}
