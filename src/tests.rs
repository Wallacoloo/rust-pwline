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
