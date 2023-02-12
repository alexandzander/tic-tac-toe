mod game;

fn main() {
    let mut b = game::Board::new();
    b.disp();
    while !b.is_done() {
        b.play();
        b.disp();
    }
    println!("Winner is {:?}", b.winner());
}
