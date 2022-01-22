use std::collections::HashSet;
use std::fmt;

mod board;

pub use board::Board;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Player { X, O }

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

pub enum Status {
    Winner(Player),
    Tie,
    InProgress,
}


pub struct STTT {
    player: Player,
    board: Board,
    valid_boards: HashSet<usize>,
}

// Public interface
impl STTT {
    pub fn new() -> STTT {
        let mut valid_boards = HashSet::new();
        // in the beginning, every board is valid!
        for i in 0..9 {
            valid_boards.insert(i);
        }
        STTT {
            player: Player::X,
            board: Board::new(),
            valid_boards,
        }
    }

    pub fn player(&self) -> Player { self.player }
    pub fn board(&self) -> Board { self.board }

    pub fn play(&mut self, player: Player, position: u32) -> Result<Status, &str> {
        // Step 1: Check if valid play
        if player != self.player {
            return Err("It's not your turn!");
        }

        if position >= 81 {
            return Err("Square out of limits");
        }
        let position = position as usize;
        let board_idx = position / 9;
        let tile_idx = position % 9;

        if !self.valid_boards.contains(&board_idx) {
            return Err("You cannot play in that board!");
        }

        if let Err(msg) = self.board.play(board_idx, tile_idx, self.player) {
            return Err(msg);
        }

        if let Some(winner) = Board::check_winner(&self.board.metaboard()) {
            assert!(winner == player);
            return Ok(Status::Winner(winner));
        }

        // Step 3: Prepare next move
        self.valid_boards.clear();
        let next_board = position % 9;
        if self.board.is_open(next_board) {
            self.valid_boards.insert(next_board);
        } else {
            for board in 0..9 {
                if self.board.is_open(board) {
                    self.valid_boards.insert(board);
                }
            }
        }

        println!("Valid boards: {:?}", self.valid_boards);

        if self.valid_boards.is_empty() {
            return Ok(Status::Tie);
        }
        
        self.player = self.next_player();
        
        Ok(Status::InProgress)
    }


    fn next_player(&self) -> Player {
        match self.player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

