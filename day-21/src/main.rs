struct DeterministicDieState {
    roll: usize
}

impl Iterator for DeterministicDieState {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.roll += 1;
        if self.roll > 100 {
            self.roll = 1;
        }

        Some(self.roll)
    }
}

// Represent 1-10 as 0-9 for easy math
fn move_player(player_location: usize, amount: usize) -> usize {
    match (player_location + amount) % 10 {
        0 => 10,
        i => i
    }
}

fn part_one(mut player_one_location: usize, mut player_two_location: usize) -> usize {
    let mut die = DeterministicDieState { roll: 0 };
    let mut roll_count = 0;
    let mut player_one_points = 0;
    let mut player_two_points = 0;
    let mut current_player = 0;

    while player_one_points < 1000 && player_two_points < 1000 {
        let roll = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();

        match current_player {
            0 => {
                player_one_location = move_player(player_one_location, roll);
                player_one_points += player_one_location;
            },
            _ => {
                player_two_location = move_player(player_two_location, roll);
                player_two_points += player_two_location;
            }
        }

        roll_count += 3;
        current_player = (current_player + 1) % 2;
    }

    let losing_points = if player_one_points > player_two_points { player_two_points } else { player_one_points };
    losing_points * roll_count
}

fn main() {
    println!("Day 21 Part 1: {}", part_one(4, 2));
}

#[test]
fn test_day_one() {
    assert_eq!(part_one(4, 8), 739785);
}
