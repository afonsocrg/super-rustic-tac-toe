use super::player::Player;

pub enum Status {
    Winner(Player),
    Tie,
    InProgress,
}
