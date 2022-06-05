use crate::devices::DeviceInfoProvider;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub devices: Vec<String>,
}

pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn devices<'a>(&self, room: &'a Room) -> &'a Vec<String> {
        &room.devices
    }

    pub fn create_report(&self, device_info_provider: &dyn DeviceInfoProvider) -> String {
        let mut entries = vec![];
        for room in &self.rooms {
            for device_name in &room.devices {
                entries.push(device_info_provider.info(&room.name, &device_name));
            }
        }

        entries.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rooms() {
        let room1 = Room {
            name: String::from("Зал"),
            devices: vec![String::from("socket")],
        };
        let room2 = Room {
            name: String::from("Кухня"),
            devices: vec![String::from("socket"), String::from("thermo")],
        };
        let house = SmartHouse::new(String::from("house"), vec![room1, room2]);
        let rooms = house.get_rooms();
        assert_eq!(rooms.len(), 2);
    }
}
