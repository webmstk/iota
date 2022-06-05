mod providers;

pub use providers::{BorrowingDeviceInfoProvider, DeviceInfoProvider, OwningDeviceInfoProvider};

pub struct SmartSocket {
    pub state: bool,
}

pub struct SmartThermometer {
    pub state: bool,
}
