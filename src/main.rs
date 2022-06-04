#![allow(dead_code)]

struct Room {
    name: String,
    devices: Vec<String>,
}

struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn devices<'a>(&self, room: &'a Room) -> &'a Vec<String> {
        &room.devices
    }

    fn create_report(&self, device_info_provider: &dyn DeviceInfoProvider) -> String {
        let mut entries = vec![];
        for room in &self.rooms {
            for device_name in &room.devices {
                entries.push(device_info_provider.info(&room.name, &device_name));
            }
        }

        entries.join("\n")
    }
}

trait DeviceInfoProvider {
    fn info(&self, room_name: &str, device_name: &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {
    state: bool,
}
struct SmartThermometer {
    state: bool,
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
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

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket { state: true };
    let socket2 = SmartSocket { state: false };
    let thermo = SmartThermometer { state: true };

    // Инициализация дома
    let room1 = Room {
        name: String::from("Зал"),
        devices: vec![String::from("socket")],
    };
    let room2 = Room {
        name: String::from("Кухня"),
        devices: vec![String::from("socket"), String::from("thermo")],
    };
    let house = SmartHouse::new(String::from("house"), vec![room1, room2]);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
