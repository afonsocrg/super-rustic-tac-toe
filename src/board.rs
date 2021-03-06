use std::fmt;

use super::Player;

#[derive(Copy, Clone)]
/// Represents a valid position in the board
pub struct Position {
    board_idx: usize,
    tile_idx: usize,
}

impl Position {
    /// Creates a valid position from an absolute index in the board.
    ///
    /// The absolute numeration is as follows:
    /// ```text
    ///                |               |
    ///    0 | 1 | 2   |   9 | 10| 11  |   18| 19| 20
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    3 | 4 | 5   |   12| 13| 14  |   21| 22| 23
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    6 | 7 | 8   |   15| 16| 17  |   24| 25| 26
    ///                |               |
    /// ---------------+---------------+---------------
    ///                |               |
    ///    27| 28| 29  |   36| 37| 38  |   45| 46| 47
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    30| 31| 32  |   39| 40| 41  |   48| 49| 50
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    33| 34| 35  |   42| 43| 44  |   51| 52| 53
    ///                |               |
    /// ---------------+---------------+---------------
    ///                |               |
    ///    54| 55| 56  |   63| 64| 65  |   72| 73| 74
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    57| 58| 59  |   66| 67| 68  |   75| 76| 77
    ///   ---+---+---  |  ---+---+---  |  ---+---+---
    ///    60| 61| 62  |   69| 70| 71  |   78| 79| 80
    ///                |               |
    /// ```
    ///
    /// # Error
    ///
    /// If the given absolute position is out of range (smaller than 0 or greater than 80),
    /// this method returns an error
    ///
    /// # Examples
    /// ```
    /// use sttt::Position;
    ///
    /// let pos = Position::from_absolute(42).unwrap();
    /// ```
    pub fn from_absolute(pos: usize) -> Result<Position, &'static str> {
        if pos >= 81 {
            return Err("Position outside of board");
        }

        Ok(Position {
            board_idx: pos / 9,
            tile_idx:  pos % 9,
        })
    }

    /// Returns the index of the small board in the metaboard that corresponds
    /// to this position
    ///
    /// # Examples
    ///
    /// ```
    /// use sttt::Position;
    ///
    /// let pos = Position::from_absolute(42).unwrap();
    /// assert_eq!(pos.board_idx(), 4);
    /// ```
    pub fn board_idx(&self) -> usize { self.board_idx }

    /// Returns the index of the tile in the small board corresponding to the position
    ///
    /// # Examples
    ///
    /// ```
    /// use sttt::Position;
    ///
    /// let pos = Position::from_absolute(42).unwrap();
    /// assert_eq!(pos.tile_idx(), 6);
    /// ```
    pub fn tile_idx(&self) -> usize { self.tile_idx }
}

#[derive(Copy, Clone)]
/// Represents the Super Tic-Tac-Toe board.
/// It has 9 Tic-Tac-Toe boards (also called small boards) in a
/// 3x3 grid (also called metaboard).
pub struct Board {
    board: [[Option<Player>;9];9],
    metaboard: [Option<Player>;9],
}

impl Board {
    /// Creates an empty `Board`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use sttt::Board;
    ///
    /// let mut board = Board::new();
    /// ```
    pub fn new() -> Board {

        Board {
            board: [[None; 9]; 9],
            metaboard: [None; 9],
        }
    }

    /// Returns a copy of the metaboard
    ///
    /// # Examples
    /// 
    /// ```
    /// use sttt::{Board, Player};
    ///
    /// let mut board = Board::new();
    /// let metaboard = board.metaboard();
    /// ```
    pub fn metaboard(&self) -> [Option<Player>; 9] {
        return self.metaboard;
    }

    /// Inserts a move from a given player in the board.
    /// 
    /// If that player wins the small board, the metaboard will 
    /// have that player in the position corresponding to the closed board
    ///
    /// # Errors
    ///
    /// If board_idx or tile_idx are greater or equal than `9`, that move is invalid
    /// and an error is returned.
    ///
    /// If the given position corresponds to an already played tile, 
    /// an error is returned as well.
    ///
    /// # Examples
    /// 
    /// ```
    /// use sttt::{Board,Player, Position};
    ///
    /// let p1 = Position::from_absolute(0).unwrap();
    /// let p2 = Position::from_absolute(1).unwrap();
    /// let p3 = Position::from_absolute(2).unwrap();
    ///
    /// let mut board = Board::new();
    /// board.play(Player::X, p1);
    /// board.play(Player::X, p2);
    /// board.play(Player::X, p3);
    /// assert_eq!(board.metaboard(),  [Some(Player::X), None, None, 
    ///                                     None, None, None, 
    ///                                     None, None, None]);
    /// ```
    pub fn play(
        &mut self,
        player: Player,
        position: Position,
    ) -> Result<(), &'static str> {

        let board_idx = position.board_idx();
        let tile_idx = position.tile_idx();

        if self.board[board_idx][tile_idx].is_some() {
            return Err("That square is not empty");
        }

        self.board[board_idx][tile_idx] = Some(player);

        if let Some(board_winner) = Board::check_winner(&self.board[board_idx]) {
            assert!(board_winner == player);
            println!("{} wins board {}!!", board_winner, board_idx);

            self.metaboard[board_idx] = Some(player);
        }

        Ok(())
    }

    /// Returns `true` if there are still valid plays in the given board.
    /// If the board already has a winner or has every tile played, then
    /// this function returns false.
    ///
    /// # Examples
    /// 
    /// ```
    /// use sttt::{Board, Player, Position};
    ///
    /// let p1 = Position::from_absolute(0).unwrap();
    /// let p2 = Position::from_absolute(1).unwrap();
    /// let p3 = Position::from_absolute(2).unwrap();
    ///
    /// let mut board = Board::new();
    /// board.play(Player::X, p1);
    /// board.play(Player::X, p2);
    /// assert_eq!(board.is_open(0),  true);
    /// board.play(Player::X, p3);
    /// assert_eq!(board.is_open(0),  false);
    /// ```
    pub fn is_open(&self, board_idx: usize) -> bool {
        assert!(board_idx < 9);

        // nobody has won this board
        self.metaboard[board_idx].is_none() &&
        // still has empty squares
        self.board[board_idx].iter()
            .filter(|x| x.is_none())
            .count() > 0
    }

    /// Tic-Tac-Toe logic to check if a 3x3 board has a winner
    ///
    /// # Examples
    /// 
    /// ```
    /// use sttt::{Board, Player};
    ///
    /// let mut ttt: [Option<Player>; 9] = [None; 9];
    /// ttt[0] = Some(Player::X);
    /// ttt[1] = Some(Player::X);
    /// assert_eq!(Board::check_winner(&ttt),  None);
    /// ttt[2] = Some(Player::X);
    /// assert_eq!(Board::check_winner(&ttt),  Some(Player::X));
    /// ```
    pub fn check_winner(board: &[Option<Player>;9]) -> Option<Player> {
        // Check rows
        for row in 0..3 {
            let row_base = row * 3;
            if  board[row_base    ] == board[row_base + 1] &&
                board[row_base + 1] == board[row_base + 2] &&
                board[row_base    ].is_some() {
                    return board[row_base];
            }
        }

        // Check cols
        for col in 0..3 {
            if  board[col    ] == board[col + 3] &&
                board[col + 3] == board[col + 6] &&
                board[col    ].is_some() {
                    return board[col];
            }
        }

        // Check diagonals
        if  board[4].is_some() && (
                board[0] == board[4] && board[4] == board[8] ||
                board[2] == board[4] && board[4] == board[6]
            ) {
            return board[4];
        }

        None
    }
}

impl fmt::Display for Board {
    /*
     *                 |               |
     *     0 | 1 | 2   |   9 | 10| 11  |   18| 19| 20
     *    ---+---+---  |  ---+---+---  |  ---+---+--- 
     *     3 | 4 | 5   |   12| 13| 14  |   21| 22| 23
     *    ---+---+---  |  ---+---+---  |  ---+---+---
     *     6 | 7 | 8   |   15| 16| 17  |   24| 25| 26
     *                 |               |
     *  ---------------+---------------+---------------             Meta Board:
     *                 |               | 
     *     27| 28| 29  |   36| 37| 38  |   45| 46| 47                0 | 1 | 2
     *    ---+---+---  |  ---+---+---  |  ---+---+---               ---+---+---
     *     30| 31| 32  |   39| 40| 41  |   48| 49| 50                3 | 4 | 5
     *    ---+---+---  |  ---+---+---  |  ---+---+---               ---+---+---
     *     33| 34| 35  |   42| 43| 44  |   51| 52| 53                6 | 7 | 8
     *                 |               |
     *  ---------------+---------------+---------------
     *                 |               |
     *     54| 55| 56  |   63| 64| 65  |   72| 73| 74
     *    ---+---+---  |  ---+---+---  |  ---+---+---
     *     57| 58| 59  |   66| 67| 68  |   75| 76| 77
     *    ---+---+---  |  ---+---+---  |  ---+---+---
     *     60| 61| 62  |   69| 70| 71  |   78| 79| 80
     *                 |               |
     *
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const BIG_ROW_EMPTY: &str = "               |               |";
        const BIG_ROW_SEP: &str   = "---------------+---------------+---------------";
        const SMALL_ROW_SEP: &str =   "---+---+---";
        const METABOARD_SEP: &str = "              ";

        let mut res = String::new();

        for big_row in 0..3 {
            res.push_str(BIG_ROW_EMPTY);
            res.push('\n');

            for small_row in 0..3 {
                // Print values of entire big row
                for big_col in 0..3 {
                    res.push_str("  ");

                    for small_col in 0..3 {
                        // let idx = big_row * 27 + big_col * 9 + small_row * 3 + small_col;
                        let board_idx = big_row * 3 + big_col;
                        let position_idx = small_row * 3 + small_col;
                        
                        match self.board[board_idx][position_idx] {
                            None => res.push_str("   "),
                            Some(p) => res.push_str(&format!(" {} ", p)[..]),
                        };

                        if small_col < 2 {
                            res.push('|');
                        }
                    }

                    if big_col < 2{
                        res.push_str("  |");
                    }

                }

                // metaboard data
                if big_row == 1 {
                    res.push_str(METABOARD_SEP);
                    for small_col in 0..3 {
                        let idx = small_row * 3 + small_col;
                        match self.metaboard[idx] {
                            None => res.push_str("   "),
                            Some(p) => res.push_str(&format!(" {} ", p)[..]),
                        };
                        if small_col < 2 {
                            res.push('|');
                        }
                    }
                }

                // Print separator
                if small_row < 2 {
                    res.push('\n');
                    for big_col in 0..3 {
                        res.push_str("  ");
                        res.push_str(SMALL_ROW_SEP);
                        if big_col < 2{
                            res.push_str("  |");
                        }
                    }

                    // metaboard separators
                    if big_row == 1 {
                        res.push_str(METABOARD_SEP);
                        res.push_str(SMALL_ROW_SEP);

                    }
                }
                res.push('\n');
            }

            res.push_str(BIG_ROW_EMPTY);
            res.push('\n');

            if big_row < 2 {
                res.push_str(BIG_ROW_SEP);

                // metaboard title
                if big_row == 0 {
                    res.push_str("             metaboard");
                }

                res.push('\n');
            }
        }

        write!(f, "{}", res)
    }
}

