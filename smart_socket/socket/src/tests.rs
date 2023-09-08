#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command_on() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::On);
        assert_eq!(response, Response::Ok);
        assert_eq!(smart_socket.enabled, true);
    }

    #[test]
    fn test_process_command_off() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::Off);
        assert_eq!(response, Response::Ok);
        assert_eq!(smart_socket.enabled, false);
    }

    #[test]
    fn test_process_command_is_on_enabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = true;
        let response = smart_socket.process_command(Command::IsOn);
        assert_eq!(response, Response::Enabled);
    }

    #[test]
    fn test_process_command_is_on_disabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = false;
        let response = smart_socket.process_command(Command::IsOn);
        assert_eq!(response, Response::Disabled);
    }

    #[test]
    fn test_process_command_get_value_enabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = true;
        let response = smart_socket.process_command(Command::GetValue);
        assert_eq!(response, Response::Value(SmartSocket::POWER_ON));
    }

    #[test]
    fn test_process_command_get_value_disabled() {
        let mut smart_socket = SmartSocket::default();
        smart_socket.enabled = false;
        let response = smart_socket.process_command(Command::GetValue);
        assert_eq!(response, Response::Value(SmartSocket::POWER_OFF));
    }

    #[test]
    fn test_process_command_unknown() {
        let mut smart_socket = SmartSocket::default();
        let response = smart_socket.process_command(Command::Unknown);
        assert_eq!(response, Response::Unknown);
    }
}
