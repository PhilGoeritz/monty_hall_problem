use rand::Rng;

enum WinningDoor {
    Chosen,
    Other,
}

fn main() {
    let mut chosen_won: u32 = 0;
    let mut other_won: u32 = 0;

    for _ in 0..1000 {
        match monty_hall_problem() {
            WinningDoor::Chosen => chosen_won += 1,
            WinningDoor::Other => other_won += 1,
        };
    }

    println!("Chosen won {chosen_won} times.");
    println!("Other won {other_won} times.");
}

fn monty_hall_problem() -> WinningDoor {
    let door_with_goat = rand::thread_rng().gen_range(0..=2);
    let chosen_door = rand::thread_rng().gen_range(0..=2);

    if door_with_goat == chosen_door {
        return WinningDoor::Chosen;
    }

    // more logic isnt needed here, because if the chosen door isnt the door with the goat,
    // then the other door is always the winning door
    WinningDoor::Other
}
