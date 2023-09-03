use super::*;

#[test]
fn test_get_demo_temperature() {
    let temperature = get_demo_temperature(0.0);
    assert_eq!(temperature, 30.0);
}
