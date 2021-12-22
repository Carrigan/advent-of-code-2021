use std::collections::HashMap;

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

#[derive(PartialEq, Eq, Hash, Debug)]
struct WorldState {
    player_one_location: usize,
    player_two_location: usize,
    player_one_points: usize,
    player_two_points: usize,
    player_one_turn: bool
}

enum AdvanceResult {
    PlayerOneWin,
    PlayerTwoWin,
    NoWin(WorldState)
}

impl WorldState {
    fn advance(&self, distance: usize) -> AdvanceResult {
        match self.player_one_turn {
            true => {
                let player_one_location = move_player(self.player_one_location, distance);
                let player_one_points = player_one_location + self.player_one_points;

                if self.player_one_points >= 21 { return AdvanceResult::PlayerOneWin; }

                AdvanceResult::NoWin(WorldState {
                    player_one_location,
                    player_one_points,
                    player_one_turn: false,
                    player_two_location: self.player_two_location,
                    player_two_points: self.player_two_points
                })
            },
            false => {
                let player_two_location = move_player(self.player_two_location, distance);
                let player_two_points = player_two_location + self.player_two_points;

                if self.player_two_points >= 21 { return AdvanceResult::PlayerTwoWin; }

                AdvanceResult::NoWin(WorldState {
                    player_two_location,
                    player_two_points,
                    player_one_turn: true,
                    player_one_location: self.player_one_location,
                    player_one_points: self.player_one_points
                })
            }
        }
    }
}

fn wins_for(world: WorldState, stored_wins: &mut HashMap<WorldState, (u128, u128)>) -> (u128, u128) {
    if let Some(&result) = stored_wins.get(&world) {
        return result;
    }

    let universe_dies = [
        (1 + 1 + 1), (1 + 1 + 2), (1 + 1 + 3),
        (1 + 2 + 1), (1 + 2 + 2), (1 + 2 + 3),
        (1 + 3 + 1), (1 + 3 + 2), (1 + 3 + 3),
        (2 + 1 + 1), (2 + 1 + 2), (2 + 1 + 3),
        (2 + 2 + 1), (2 + 2 + 2), (2 + 2 + 3),
        (2 + 3 + 1), (2 + 3 + 2), (2 + 3 + 3),
        (3 + 1 + 1), (3 + 1 + 2), (3 + 1 + 3),
        (3 + 2 + 1), (3 + 2 + 2), (3 + 2 + 3),
        (3 + 3 + 1), (3 + 3 + 2), (3 + 3 + 3),
    ];

    let result = universe_dies.iter().fold((0, 0), |(p1_wins, p2_wins), &roll|
        match world.advance(roll) {
            AdvanceResult::PlayerOneWin => (p1_wins + 1, p2_wins),
            AdvanceResult::PlayerTwoWin => (p1_wins, p2_wins + 1),
            AdvanceResult::NoWin(new_world) => {
                let (p1, p2) = wins_for(new_world, stored_wins);
                (p1_wins + p1, p2_wins + p2)
            }
        }
    );

    stored_wins.insert(world, result);

    result
}

fn part_two(mut player_one_location: usize, mut player_two_location: usize) -> u128 {
    let mut stored_wins = HashMap::new();
    let initial_state = WorldState {
        player_one_location,
        player_two_location,
        player_one_points: 0,
        player_two_points: 0,
        player_one_turn: true
    };

    let (p1, p2) = wins_for(initial_state, &mut stored_wins);

    if p1 > p2 { p1 / (27 * 27) } else { p2 / (27 * 27) }
}

fn main() {
    println!("Day 21 Part 1: {}", part_one(4, 2));
    println!("Day 21 Part 2: {}", part_two(4, 2));
}

#[test]
fn test_day_one() {
    assert_eq!(part_one(4, 8), 739785);
}

#[test]
fn test_day_two() {
    assert_eq!(part_two(4, 8), 444_356_092_776_315);
}
