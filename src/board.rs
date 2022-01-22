use std::fmt;

use super::Player;

#[derive(Copy, Clone)]
pub struct Board {
    board: [[Option<Player>;9];9],
    metaboard: [Option<Player>;9],
}

impl Board {
    pub fn new() -> Board {

        Board {
            board: [[None; 9]; 9],
            metaboard: [None; 9],
        }
    }

    pub fn metaboard(&self) -> [Option<Player>; 9] {
        return self.metaboard;
    }

    pub fn play(&mut self, board_idx: usize, tile_idx: usize, player: Player) -> Result<(), &'static str> {
        if board_idx >= 9 || tile_idx >= 9 {
            return Err("Position out of board");
        }

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

    pub fn is_open(&self, board_idx: usize) -> bool {
        assert!(board_idx < 9);

        // nobody has won this board
        self.metaboard[board_idx].is_none() &&
        // still has empty squares
        self.board[board_idx].iter()
            .filter(|x| x.is_none())
            .count() > 0
    }

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

