struct Rectangle{
    width: u32,
    height: u32
}

struct Square{
    length: u32,
}

struct RightAngledTriange{
    width: u32,
    height: u32
}

fn get_rectangle_area(rectangle: &Rectangle) -> u32
{
    return rectangle.width * rectangle.height
}

fn get_square_area(square: &Square) -> u32
{
    return square.length * square.length
}

fn get_right_angled_triangle_area(triangle: &RightAngledTriange) -> f32
{
    return (triangle.width) as f32 * (triangle.height) as f32 * 0.5
}

fn main() {
    let rect = Rectangle{width: 3, height: 5};
    let square = Square{length: 5};
    let triangle = RightAngledTriange{width: 5, height: 3};
    println!("A 3 x 5 right angled triangle is {}cm^2", get_right_angled_triangle_area(&triangle));
    println!("A 3 x 5 rectangle is {}cm^2", get_rectangle_area(&rect));
    println!("A 5 x 5 square is {}cm^2", get_square_area(&square));
}
