/// string move and copy

fn main() {

    let mut mhouse=String::from("my house ");

    make_semi(&mut mhouse);
    make_semi(&mut mhouse);
    println!("{mhouse}");

}

fn make_semi(house: &mut String){
    house.push_str(" - semi");
}