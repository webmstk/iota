use super::{SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn info(&self, room_name: &str, device_name: &str) -> String;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        match device_name {
            "socket" => format!("{} {}: {}", room_name, device_name, self.socket.state),
            _ => format!("{} {}: no device in the room", room_name, device_name),
        }
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        match device_name {
            "socket" => format!("{} {}: {}", room_name, device_name, self.socket.state),
            "thermo" => format!("{} {}: {}", room_name, device_name, self.thermo.state),
            _ => format!("{} {}: no device in the room", room_name, device_name),
        }
    }
}
