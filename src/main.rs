fn main() {
    let string = std::fs::read_to_string("../../cube.obj").unwrap();
    let parsed = bop::parse(string).unwrap();

    println!("{:?}", parsed.0);
    println!("{:?}", parsed.1);
}
