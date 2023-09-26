struct Colour(u8, u8, u8);

fn main() {
    let red = Colour(255, 0, 0);
    if red.0 == 255{
        println!("yes");
    }
}
