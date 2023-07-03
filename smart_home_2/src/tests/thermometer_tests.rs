#[cfg(test)]
mod tests {
    use crate::types::device::Device;
    use crate::types::thermometer::Thermometer;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer::new(
            "Living Room".to_string(),
            "Thermometer".to_string(),
            "On".to_string(),
        );

        assert_eq!(thermometer.get_room(), "Living Room");
        assert_eq!(thermometer.get_name(), "Thermometer");
        assert_eq!(thermometer.get_value(), "On");
    }
}
