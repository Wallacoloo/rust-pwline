use super::pwline::PwLine;

#[test]
fn test_const() {
    let x = PwLine::from_const(2.0f32);
    // test interior
    for i in 0u32..21u32 {
        assert_eq!(x.get(i), 2.0f32);
    }
}

#[test]
fn test_oob() {
    let mut x = PwLine::new();
    x.add_pt(-10i32, 2.0f32);
    x.add_pt(20i32, -2.0f32);

    // Immediately OOB
    assert_eq!(x.get(-11i32), 2.0f32);
    assert_eq!(x.get(21i32), -2.0f32);

    // Farther OOB
    assert_eq!(x.get(-999i32), 2.0f32);
    assert_eq!(x.get(999i32), -2.0f32);
}

#[test]
fn test_empty() {
    let x = PwLine::<u32, f32>::new();
    // Empty functions default to Zero
    assert_eq!(x.get(11u32), 0.0f32);
}


#[test]
fn test_two_seg() {
    let mut x = PwLine::new();
    x.add_pt(0u32, 2.0f32);
    x.add_pt(20u32, 4.0f32);
    x.add_pt(40u32, 3.0f32);

    assert_eq!(x.get(0u32), 2.0f32);
    assert_eq!(x.get(5u32), 2.5f32);
    assert_eq!(x.get(10u32), 3.0f32);
    assert_eq!(x.get(15u32), 3.5f32);
    assert_eq!(x.get(20u32), 4.0f32);
    assert_eq!(x.get(25u32), 3.75f32);
    assert_eq!(x.get(30u32), 3.5f32);
    assert_eq!(x.get(35u32), 3.25f32);
    assert_eq!(x.get(40u32), 3.0f32);
}

#[test]
fn test_get_consec() {
    let mut x = PwLine::new();
    x.add_pt(0u32, 2.0f32);
    x.add_pt(2u32, 4.0f32);
    x.add_pt(4u32, 3.0f32);
    x.add_pt(6u32, 0.0f32);
    let got : Vec<f32> = x.get_consec(1u32).take(6).collect();
    assert_eq!(got, vec![/* 1 */ 3.0f32, /* 2 */ 4.0f32, /* 3 */ 3.5f32,
        /* 4 */ 3.0f32, /* 5 */ 1.5f32, /* 6 */ 0.0f32]);
}
