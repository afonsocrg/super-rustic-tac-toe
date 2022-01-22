use std::collections::HashSet;
use std::fmt;

mod board;

pub use board::{Board,Position};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Represents the possible players in a 
/// Super Tic-Tac-Toe game: `X` and `O`.
pub enum Player { X, O }

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// Represents the possible statuses of a game.
pub enum Status {
    /// Represents that `Player` has won the game.
    Winner(Player),
    /// Represents that the game ended in a Tie.
    Tie,
    /// Represents that the game is still in progress.
    InProgress,
}


pub struct STTT {
    player: Player,
    board: Board,
    valid_boards: HashSet<usize>,
}

impl STTT {
    /// Creates a new Super Tic-Tac-Toe game, with an empty board.
    /// The first player is `Player::X` and `X` can play in any big board,
    /// in the first move.
    ///
    /// # Examples
    ///
    /// ```
    /// use sttt::STTT;
    ///
    /// let mut game = STTT::new();
    /// ```
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

    /// Returns the next player to play
    ///
    /// # Examples
    ///
    /// ```
    /// use sttt::{STTT, Player, Position};
    ///
    /// let p1 = Position::from_absolute(0).unwrap();
    ///
    /// let mut game = STTT::new();
    /// assert_eq!(game.player(), Player::X);
    /// game.play(Player::X, p1);
    /// assert_eq!(game.player(), Player::O);
    /// ```
    pub fn player(&self) -> Player { self.player }

    /// Returns a copy of the game board
    pub fn board(&self) -> Board { self.board }

    /// Makes player play at a given position.
    ///
    /// Returns the game `Status` resulting from this play in case of success.
    ///
    /// The next player to make a move swaps at each successful call to this function.
    ///
    /// # Errors
    ///
    /// This function returns an error if a player plays in the other's turn,
    /// if the given position is out of bounds, or if the play is invalid in the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use sttt::{STTT,Player, Position};
    ///
    /// let p1 = Position::from_absolute(0).unwrap();
    /// let p2 = Position::from_absolute(1).unwrap();
    /// let p3 = Position::from_absolute(9).unwrap();
    ///
    /// let mut game = STTT::new();
    /// game.play(Player::X, p1).unwrap();
    /// game.play(Player::O, p2).unwrap();
    /// game.play(Player::X, p3).unwrap();
    /// ```
    pub fn play(&mut self, player: Player, position: Position) -> Result<Status, &str> {
        // Step 1: Check if valid play
        if player != self.player {
            return Err("It's not your turn!");
        }
        if !self.valid_boards.contains(&position.board_idx()) {
            return Err("You cannot play in that board!");
        }

        // Step 2: Play the given move
        if let Err(msg) = self.board.play(self.player, position) {
            return Err(msg);
        }

        // Step 3: Check winner
        if let Some(winner) = Board::check_winner(&self.board.metaboard()) {
            assert!(winner == player);
            return Ok(Status::Winner(winner));
        }

        // Step 4: Prepare next move
        self.valid_boards.clear();
        let next_board = position.tile_idx();
        if self.board.is_open(next_board) {
            // Play in corresponding board if open
            self.valid_boards.insert(next_board);
        } else {
            // Otherwise play in every available board
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

