const DIRAC_DICES: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Default, Clone)]
struct Player {
    position: usize,
    score: usize,
    to_win: usize,
}

impl Player {
    fn init(position: usize, to_win: usize) -> Self {
        Self {
            position,
            to_win,
            ..Default::default()
        }
    }

    fn play(&mut self, value: usize) -> bool {
        self.position += value;
        self.position = (self.position - 1) % 10 + 1;
        self.score += self.position;

        self.score >= self.to_win
    }
}

fn main() {
    println!("Part 1 result: {}", part1(4, 9));
    println!("Part 2 result: {}", part2(4, 9));
}

fn part1(start_1: usize, start_2: usize) -> usize {
    let mut player_1 = Player::init(start_1, 1_000);
    let mut player_2 = Player::init(start_2, 1_000);

    let mut dice_value = 1usize;
    let mut dice_rolls = 0usize;

    let loser = loop {
        // Player 1
        let mut player_rolls = 0;
        for _ in 0..3 {
            player_rolls += dice_value;

            dice_value = dice_value % 100 + 1;
        }
        dice_rolls += 3;

        if player_1.play(player_rolls) {
            break player_2;
        }

        // Player 2
        player_rolls = 0;
        for _ in 0..3 {
            player_rolls += dice_value;

            dice_value = dice_value % 100 + 1;
        }
        dice_rolls += 3;

        if player_2.play(player_rolls) {
            break player_1;
        }
    };

    loser.score * dice_rolls
}

fn play_game(player_1: Player, player_2: Player, turn: usize) -> (usize, usize) {
    let next_turn = match turn {
        1 => 2,
        _ => 1,
    };

    let (mut player_1_universes, mut player_2_universes) = (0, 0);

    for (value, times) in DIRAC_DICES {
        let mut new_player_1 = player_1.clone();
        let mut new_player_2 = player_2.clone();

        // Select player who must play
        let mut player = match turn {
            1 => new_player_1.clone(),
            _ => new_player_2.clone(),
        };

        let win = player.play(value);

        match win {
            true => match turn {
                1 => player_1_universes += times,
                _ => player_2_universes += times,
            },
            _ => {
                match turn {
                    1 => new_player_1 = player,
                    _ => new_player_2 = player,
                }

                let (new_player_1_universes, new_player_2_universes) = play_game(new_player_1, new_player_2, next_turn);

                player_1_universes += new_player_1_universes * times;
                player_2_universes += new_player_2_universes * times;
            }
        };
    }

    (player_1_universes, player_2_universes)
}

fn part2(start_1: usize, start_2: usize) -> usize {
    let player_1 = Player::init(start_1, 21);
    let player_2 = Player::init(start_2, 21);

    let (player_1_universes, player_2_universes) = play_game(player_1, player_2, 1);

    player_1_universes.max(player_2_universes)
}

#[test]
fn test_part1() {
    assert_eq!(739785, part1(4, 8));
    assert_eq!(903630, part1(4, 9));
}

#[test]
fn test_part2() {
    assert_eq!(444356092776315, part2(4, 8));
    assert_eq!(303121579983974, part2(4, 9));
}
