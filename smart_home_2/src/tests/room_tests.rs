#[cfg(test)]
mod tests {
    use crate::types::device::Device;
    use crate::types::room::Room;
    use crate::types::socket::Socket;

    struct MockDevice {
        name: String,
    }

    impl Device for MockDevice {
        fn get_room(&self) -> String {
            unimplemented!()
        }
        fn get_name(&self) -> String {
            self.name.clone()
        }
        fn get_value(&self) -> String {
            unimplemented!()
        }
    }

    #[test]
    fn test_room() {
        let socket = Socket::new(
            "Living Room".to_string(),
            "Socket".to_string(),
            "On".to_string(),
        );
        let device1 = Box::new(MockDevice {
            name: "Device1".to_string(),
        });
        let device2 = Box::new(MockDevice {
            name: "Device2".to_string(),
        });
        let mut room = Room::new("Living Room".to_string(), socket, vec![device1, device2]);

        assert_eq!(room.get_name(), "Living Room");
        assert_eq!(room.get_socket().get_name(), "Living Room_Socket");

        let device = room.get_device("Device1".to_string());
        assert!(device.is_some());
        assert_eq!(device.unwrap().get_name(), "Device1");

        let devices = room.get_devices();
        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].get_name(), "Device1");
        assert_eq!(devices[1].get_name(), "Device2");

        let device3 = Box::new(MockDevice {
            name: "Device3".to_string(),
        });
        room.add_device(device3);
        let devices = room.get_devices();
        assert_eq!(devices.len(), 3);
        assert_eq!(devices[2].get_name(), "Device3");
    }
}
