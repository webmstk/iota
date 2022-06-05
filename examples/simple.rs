use iota::devices::{
    BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartSocket, SmartThermometer,
};
use iota::house::{Room, SmartHouse};

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
