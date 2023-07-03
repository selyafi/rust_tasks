#[cfg(test)]
mod tests {
    use crate::types::socket::Socket;

    #[test]
    fn test_socket() {
        let socket = Socket::new(
            "Living Room".to_string(),
            "Socket".to_string(),
            "On".to_string(),
        );

        assert_eq!(socket.get_name(), "Living Room_Socket");
        assert_eq!(socket.get_value(), "On");
        assert_eq!(socket._get_room(), "Living Room");
    }
}
