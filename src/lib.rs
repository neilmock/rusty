extern crate rusty_lang as lang;
extern crate rusty_math as math;

#[test]
fn generics_largest() {
    let v = lang::generics::largest(&[1,2,3,4,5]);

    assert_eq!(5, v);
}

#[test]
fn math_modexp_sf() {
    assert_eq!(445, math::modexp::sf(4, 13, 497));
}

#[test]
fn math_modexp_meff() {
    assert_eq!(11564, math::modexp::meff(58, 31, 13343));
}
