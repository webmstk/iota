use iota::house::{Room, SmartHouse};

#[test]
fn test_devices() {
    let room = Room {
        name: String::from("Зал"),
        devices: vec![String::from("socket")],
    };
    let house = SmartHouse::new(String::from("house"), vec![room]);
    let room = house.get_rooms().first().unwrap();
    assert_eq!(house.devices(room).len(), 1);
}
