use std::io::{self, Write};

mod sttt;

fn main() {
    println!("Welcome to Super Tic Tac Toe!");

    let mut game = sttt::STTT::new();

    loop {
        println!("{}", game);

        // loop until valid play
        loop {
            print!(" --> {} to play: ", game.player());
            io::stdout().flush().expect("IO Error");

            let mut square = String::new();
            io::stdin()
                .read_line(&mut square)
                .expect("Failed to read from stdin");

            let square: u32 = match square.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };

            match game.play(game.player(), square) {
                Ok(status) => match status {
                    sttt::Status::Winner(p) => {
                        println!("{}", game);
                        println!("{} wins!", p);
                        return;
                    },
                    sttt::Status::Tie => {
                        println!("{}", game);
                        println!("Game ended in a tie");
                        return;
                    },
                    _ => break,
                },
                Err(s) => println!("Error: {}", s),
            }
        }
    }
}
