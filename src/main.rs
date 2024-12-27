use rand::Rng;

fn main() {
    print!("\t");
    for num_coins in 2..20 {
        print!("\t{num_coins}");
    }
    println!();

    for num_players in 2..=10 {
        print!("Num Players: {num_players}");
        for num_coins in 2..=20 {
            let mut average_rounds = 0.0;
            for _ in 0..1000 {
                average_rounds += run_dreidel_game(num_players, num_coins);
            }
            average_rounds /= 1000.0;
            print!("\t{average_rounds:.1}");
        }
        println!();
    }
}

fn run_dreidel_game(num_players: usize, num_coins_per_player: usize) -> f32 {
    let mut rng = rand::thread_rng();
    let mut player_coins = vec![num_coins_per_player - 1; num_players];

    let mut num_rounds = 0.0;
    let mut pot: usize = num_players;
    let mut num_active_players = num_players;
    while num_active_players > 1 {
        for i in 0..player_coins.len() {
            if player_coins[i] == 0 {
                continue;
            }

            match rng.gen_range(0..4) {
                0 => (),
                1 => {
                    player_coins[i] -= 1;
                    pot += 1;
                    if player_coins[i] == 0 {
                        num_active_players -= 1;
                    }
                }
                2 => {
                    player_coins[i] += (pot + 1) / 2;
                    pot /= 2;
                }
                3 => {
                    player_coins[i] += pot;

                    pot = num_active_players;

                    for coins in player_coins.iter_mut() {
                        if *coins > 0 {
                            *coins -= 1;
                            if *coins == 0 {
                                num_active_players -= 1;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }

            num_rounds += 1.0;
        }
    }

    num_rounds
}
