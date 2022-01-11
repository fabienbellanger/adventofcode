#[derive(Debug, Default)]
struct Player {
    position: usize,
    score: usize,
}

fn main() {
    println!("Part 1 result: {}", part1(4, 9));
    println!("Part 2 result: {}", part2(4, 9));
}

fn part1(start_1: usize, start_2: usize) -> usize {
    let mut player_1 = Player::default();
    let mut player_2 = Player::default();

    player_1.position = start_1;
    player_2.position = start_2;

    let mut dice_value = 1usize;
    let mut dice_rolls = 0usize;

    loop {
        // Player 1
        for _ in 0..3 {
            dice_rolls += 1;

            player_1.position += dice_value;

            dice_value += 1;
            dice_value = (dice_value - 1) % 100 + 1;
        }
        player_1.position = (player_1.position - 1) % 10 + 1;
        player_1.score += player_1.position;

        if player_1.score >= 1_000 {
            break;
        }

        // Player 2
        for _ in 0..3 {
            dice_rolls += 1;

            player_2.position += dice_value;

            dice_value += 1;
            dice_value = (dice_value - 1) % 100 + 1;
        }
        player_2.position = (player_2.position - 1) % 10 + 1;
        player_2.score += player_2.position;

        if player_2.score >= 1_000 {
            break;
        }
    }

    match (player_1.score, player_2.score) {
        (score_1, score_2) if score_1 >= 1_000 => score_2 * dice_rolls,
        (score_1, _) => score_1 * dice_rolls,
    }
}

fn part2(_start_1: usize, _start_2: usize) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(739785, part1(4, 8));
    assert_eq!(903630, part1(4, 9));
}

#[test]
fn test_part2() {
    // assert_eq!(0, part2(get_data("test.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}
