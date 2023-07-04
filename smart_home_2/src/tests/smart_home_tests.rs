#[cfg(test)]
mod tests {
    use crate::api::smart_home;
    use crate::types::room::Room;
    use crate::types::socket::Socket;
    use crate::types::thermometer::Thermometer;

    #[test]
    fn test_smart_home() {
        let kitchen = Room::new(
            "Kitchen".to_string(),
            Socket::new(
                "Kitchen".to_string(),
                "Socket".to_string(),
                "On".to_string(),
            ),
            vec![Box::new(Thermometer::new(
                "Kitchen".to_string(),
                "Thermometer".to_string(),
                "On".to_string(),
            ))],
        );

        let mut smart_home =
            smart_home::SmartHome::new("Home".to_string(), "user".to_string(), "pass".to_string());
        smart_home.add_room(kitchen);

        assert_eq!(smart_home.get_rooms().len(), 1);

        let room = smart_home.get_room("Kitchen");
        assert!(room.is_some());
        assert_eq!(room.unwrap().get_name(), "Kitchen");
    }
}
