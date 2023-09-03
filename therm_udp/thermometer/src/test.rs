#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer::new("127.0.0.1:1234").unwrap();
        assert_eq!(thermometer.get(), 0.0);
        thermometer.value.set(25.0);
        assert_eq!(thermometer.get(), 25.0);
    }
}