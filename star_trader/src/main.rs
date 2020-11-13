mod game_board;

fn main() {
    let mut board = game_board::Board::new(15, 12, 5, 10);
    board.display()
}
