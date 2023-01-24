// A rock-paper-scissors game

mod io {
    pub struct Io;

    impl Io {
        pub fn read_line(&self) -> String {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }
    }
}

pub mod points {
    pub struct Points {
        pub player_number: u8,
        pub points: u8,
    }

    pub fn calculate_points(one_choice: String, two_choice: String) -> Points {
        let mut points: Points = Points {
            player_number: 0,
            points: 0,
        };
        if one_choice == "rock" && two_choice == "scissors" {
            points.player_number = 1;
            points.points = 1;
        } else if one_choice == "rock" && two_choice == "paper" {
            points.player_number = 2;
            points.points = 1;
        } else if one_choice == "paper" && two_choice == "rock" {
            points.player_number = 1;
            points.points = 1;
        } else if one_choice == "paper" && two_choice == "scissors" {
            points.player_number = 2;
            points.points = 1;
        } else if one_choice == "scissors" && two_choice == "paper" {
            points.player_number = 1;
            points.points = 1;
        } else if one_choice == "scissors" && two_choice == "rock" {
            points.player_number = 2;
            points.points = 1;
        }
        points
    }
}

use io::Io;
use points::calculate_points;

struct Player {
    name: String,
    points: u8,
}

fn main() {
    println!("Welcome to rock, paper scissors. What should player one be called? ");
    let mut one: Player = Player {
        name: Io.read_line().trim().to_string(),
        points: 0,
    };
    println!("What should player two be called? ");
    let mut two: Player = Player {
        name: Io.read_line().trim().to_string(),
        points: 0,
    };
    loop {
        println!("{} vs. {}", one.name, two.name);
        println!("{}: rock, paper, or scissors? ", one.name);
        let one_choice: String = Io.read_line().trim().to_string();
        println!("{}: rock, paper, or scissors? ", two.name);
        let two_choice: String = Io.read_line().trim().to_string();
        let points = calculate_points(one_choice, two_choice);
        if points.player_number == 0 {
            println!("It's a tie!");
        } else {
            println!("{} wins!", points.player_number);
            if points.player_number == 1 {
                one.points += points.points;
            } else {
                two.points += points.points;
            }
        }
        println!("Play again? (enter y or n): ");
        let play_again: String = Io.read_line().trim().to_string();
        if play_again == "n" {
            break;
        }
    }

    println!(
        "{} scored: {}\n{} scored: {}\nThanks for playing!",
        one.name, one.points, two.name, two.points
    );
}
