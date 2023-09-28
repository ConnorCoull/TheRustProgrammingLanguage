enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State{
    Alabama,
    Alaska, 
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii, 
    Idaho,
    Illinois, 
    Indiana, 
    Iowa, 
    Kansas, 
    Kentucky, 
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana, 
    Nebraska, 
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming
}

fn your_quarter_is_from(coin: Coin)
{
    if let Coin::Quarter(state) = coin {
        println!("Your quarter is from {:?}!", state);
    }
    //match coin {
    //    Coin::Quarter(state) => 
    //    {
    //       println!("Your quarter is from {:?}!", state); 
    //    }
    //    _ => println!("This coin doesn't have an associated State.")
    //}
}



fn main()
{
    let alabama_quarter = Coin::Penny;
    your_quarter_is_from(alabama_quarter);
}