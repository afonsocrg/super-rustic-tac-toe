use std::fmt;

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
