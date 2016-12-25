use super::pwline::PwLine;

#[test]
fn test_const() {
    let mut x = PwLine::new();
    x.add_pt(0u32, 2.0f32);
    x.add_pt(20u32, 2.0f32);
    // test boundaries (range should be inclusive)
    assert_eq!(x.get(0u32).unwrap(), 2.0f32);
    assert_eq!(x.get(20u32).unwrap(), 2.0f32);
    // test interior
    for i in 1..20 {
        assert_eq!(x.get(i).unwrap(), 2.0f32);
    }
}

#[test]
fn test_oob() {
    let mut x = PwLine::new();
    x.add_pt(-10i32, 2.0f32);
    x.add_pt(20i32, -2.0f32);

    // Immediately OOB
    assert_eq!(x.get(-11i32), None);
    assert_eq!(x.get(21i32), None);

    // Farther OOB
    assert_eq!(x.get(-999i32), None);
    assert_eq!(x.get(999i32), None);
}

#[test]
fn test_two_seg() {
    let mut x = PwLine::new();
    x.add_pt(0u32, 2.0f32);
    x.add_pt(20u32, 4.0f32);
    x.add_pt(40u32, 3.0f32);

    assert_eq!(x.get(0u32).unwrap(), 2.0f32);
    assert_eq!(x.get(5u32).unwrap(), 2.5f32);
    assert_eq!(x.get(10u32).unwrap(), 3.0f32);
    assert_eq!(x.get(15u32).unwrap(), 3.5f32);
    assert_eq!(x.get(20u32).unwrap(), 4.0f32);
    assert_eq!(x.get(25u32).unwrap(), 3.75f32);
    assert_eq!(x.get(30u32).unwrap(), 3.5f32);
    assert_eq!(x.get(35u32).unwrap(), 3.25f32);
    assert_eq!(x.get(40u32).unwrap(), 3.0f32);
}

#[test]
fn test_get_consec() {
    let mut x = PwLine::new();
    x.add_pt(0u32, 2.0f32);
    x.add_pt(2u32, 4.0f32);
    x.add_pt(4u32, 3.0f32);
    x.add_pt(6u32, 0.0f32);
    let got : Vec<f32> = x.get_consec(1u32).collect();
    assert_eq!(got, vec![/* 1 */ 3.0f32, /* 2 */ 4.0f32, /* 3 */ 3.5f32,
        /* 4 */ 3.0f32, /* 5 */ 1.5f32, /* 6 */ 0.0f32]);
}
