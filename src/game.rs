#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

pub struct Board {
    state: [[Option<Player>; 3]; 3],
    player: Player,
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [[None; 3]; 3],
            player: Player::X,
        }
    }

    pub fn disp(&self) {
        let mut i = 1;
        for row in self.state {
            for cell in row {
                match cell {
                    Some(Player::X) => print!("X "),
                    Some(Player::O) => print!("O "),
                    None => print!(". "),
                };
            }
            println!("      {} {} {}", i, i + 1, i + 2);
            i += 3;
        }
    }

    pub fn play(&mut self) {
        println!("\nNext move?");
        loop {
            let mut num = String::new();
            std::io::stdin()
                .read_line(&mut num)
                .expect("Failed to read line.");
            match num.trim().parse::<usize>() {
                Err(_) => println!("Not a valid input."),
                Ok(n) => {
                    if 1 <= n && n <= 9 {
                        if let None = self.state[(n - 1) / 3][(n - 1) % 3] {
                            self.state[(n - 1) / 3][(n - 1) % 3] = Some(self.player);
                            self.player = self.player.next();
                            break;
                        } else {
                            println!("Already filled");
                        }
                    } else {
                        println!("Must be an int between 1 and 9.");
                    }
                }
            };
        }
    }

    pub fn is_done(&self) -> bool {
        if let Some(_) = self.winner() {
            return true;
        }
        if self
            .state
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
        {
            return true;
        }
        false
    }

    pub fn winner(&self) -> Option<Player> {
        macro_rules! has {
            ($player:expr, $x:expr, $y:expr) => {
                self.state[$x][$y] == Some(*$player)
            };
        }

        for p in &[Player::X, Player::O] {
            for row in self.state {
                if row.iter().all(|cell| *cell == Some(*p)) {
                    return Some(*p);
                }
            }

            for col in 0..=2 {
                if has!(p, 0, col) && has!(p, 1, col) && has!(p, 2, col) {
                    return Some(*p);
                }
            }

            if has!(p, 0, 0) && has!(p, 1, 1) && has!(p, 2, 2) {
                return Some(*p);
            }

            if has!(p, 2, 0) && has!(p, 1, 1) && has!(p, 0, 2) {
                return Some(*p);
            }
        }
        None
    }
}
