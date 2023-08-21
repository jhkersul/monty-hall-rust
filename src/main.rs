mod doors;
use std::{thread, time};
use std::io::{stdout, Write};
use rand::{thread_rng, Rng};
use crate::doors::{Door, build_doors};

fn main() {
    let mut stdout = stdout();
    let mut total_games = 0;
    let mut won_change_door = 0;
    let mut won_not_change_door = 0;

    #[allow(while_true)]
    while true {
        let mut game_change_doors = Game {
            doors: build_doors(),
            guesses: vec![],
            openned: vec![],
            change_door: true,
            won: false,
        };
        game_change_doors.play();

        let mut game_not_change_doors = Game {
            doors: build_doors(),
            guesses: vec![],
            openned: vec![],
            change_door: false,
            won: false,
        };
        game_not_change_doors.play();

        total_games = total_games + 1;

        if game_change_doors.won {
            won_change_door += 1;
        }

        if game_not_change_doors.won {
            won_not_change_door += 1
        }

        print!("\rTotal Games: {:?} - Won Change Door: {:?} - Won Not Change Door: {:?} - Percentage Change Door: {:?} - Percentage Not Change Door: {:?}", total_games, won_change_door, won_not_change_door, (won_change_door as f32 / total_games as f32) * 100.0, (won_not_change_door as f32 / total_games as f32) * 100.0);

        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_millis(1));
    }
}

#[derive(Debug)]
struct Game {
    doors: Vec<Door>,
    guesses: Vec<u32>,
    openned: Vec<u32>,
    change_door: bool,
    won: bool,
}

impl Game {
    fn play(&mut self) {
        let mut guess = self.guess();

        self.open_door(guess);
        if self.change_door {
            guess = self.swap_guess(guess);
        }
        self.open_door(guess);

        self.check_won(guess);

        // println!("Won: {:?}", self.won);
        // println!("Guesses: {:?}", self.guesses);
        // println!("Openned: {:?}", self.openned);
        // println!("Doors: {:?}", self.doors);
    }

    fn check_won(&mut self, guess: u32) {
        self.won = self.check_door(guess);
    }
    

    fn guess(&mut self) -> u32 {
        let guess = self.select_random_not_openned_door();
        self.guesses.push(guess);

        return guess
    }

    fn swap_guess(&mut self, previous_guess: u32) -> u32 {
        let available_door: &Door = self.doors.iter()
            .find(|door| door.number != previous_guess && !self.openned.contains(&door.number)).unwrap();

        let new_guess = available_door.number.clone();
        self.guesses.push(new_guess);

        return new_guess;
    }

    fn open_door(&mut self, guess: u32) -> bool {
        if self.openned.len() == 2 {
            panic!("You can only open two doors");
        }

        let door_number = self.select_first_not_guessed_not_prized_door(guess);

        self.openned.push(door_number);

        return self.check_door(door_number);
    }

    fn check_door(&self, door_number: u32) -> bool {
        let door = self.doors.iter().find(|door| door.number == door_number).unwrap();
        return door.has_prize;
    }

    fn select_first_not_guessed_not_prized_door(&self, guess: u32) -> u32 {
        for door in self.doors.iter() {
            if door.number != guess && !door.has_prize && !self.openned.contains(&door.number) {
                return door.number.clone();
            }

            if door.number == 3 || (guess == 3 && door.number == 2) {
                return door.number.clone()
            }
        }

        panic!("No door found");
    }

    fn select_random_not_openned_door(&self) -> u32 {
        let mut selected_door: u32 = self.random_door();
        while self.openned.contains(&selected_door) {
            selected_door = self.random_door();
        }
        return selected_door;
    }

    fn random_door(&self) -> u32 {
        let mut rng = thread_rng();
        return rng.gen_range(1..4)
    }
}

#[cfg(test)]
mod tests {
    use crate::doors::Door;
    use crate::Game;

    #[test]
    fn test_check_door() {
        let game = mock_game(1);

        assert_eq!(game.check_door(1), true);
        assert_eq!(game.check_door(2), false);
        assert_eq!(game.check_door(3), false);
    }

    #[test]
    fn test_guess() {
        let mut game = mock_game(1);

        game.guess();

        assert_eq!(game.guesses.len(), 1);
    }

    #[test]
    fn test_guess_twice() {
        let mut game = mock_game(1);

        game.guess();
        game.guess();

        assert_eq!(game.guesses.len(), 2);
    }

    #[test]
    fn test_select_first_not_guessed_not_prized_door() {
        let game = mock_game(1);
        let guess = 1;

        let selected = game.select_first_not_guessed_not_prized_door(guess);

        assert_eq!(selected, 2);
    }

    #[test]
    fn test_select_first_not_guessed_not_prized_door_guess_2() {
        let game = mock_game(1);
        let guess = 2;

        let selected = game.select_first_not_guessed_not_prized_door(guess);

        assert_eq!(selected, 3);
    }

    #[test]
    fn test_select_first_not_guessed_not_prized_door_guess_3() {
        let game = mock_game(2);
        let guess = 3;

        let selected = game.select_first_not_guessed_not_prized_door(guess);

        assert_eq!(selected, 1);
    }
    

    fn mock_game(prize_door: u32) -> Game {
        return Game {
            doors: vec![
                Door { number: 1, has_prize: prize_door == 1 },
                Door { number: 2, has_prize: prize_door == 2 },
                Door { number: 3, has_prize: prize_door == 3 },
            ],
            guesses: vec![],
            openned: vec![],
            change_door: false,
            won: false,
        };
    }
}

