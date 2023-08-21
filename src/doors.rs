use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Door {
    pub number: u32,
    pub has_prize: bool,
}

pub fn build_doors() -> Vec<Door> {
    let mut doors = Vec::new();
    let selected_door = select_door_random();

    for i in 1..4 {
        doors.push(
            Door { number: i, has_prize: selected_door == i }
        );
    }

    return doors
}

pub fn select_door_random() -> u32 {
    let mut rng = thread_rng();
    let selected_door: u32 = rng.gen_range(1..4);
    return selected_door;
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_build_doors() {
        let doors = super::build_doors();

        assert_eq!(doors.len(), 3);
        assert_eq!(doors[0].number, 1);
        assert_eq!(doors[1].number, 2);
        assert_eq!(doors[2].number, 3);
        assert_eq!(doors.iter().filter(|door| door.has_prize).count(), 1);
    }

    #[test]
    fn test_select_door_random() {
        let selected_door = super::select_door_random();
        assert!(selected_door >= 1 && selected_door <= 3);
    }
}

