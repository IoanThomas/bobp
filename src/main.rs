fn main() {
    let obj_string = std::fs::read_to_string("../../cube.obj").expect("failed to load OBJ file");
    let (vertices, indices) = bop::parse_obj(obj_string).expect("failed to parse OBJ");

    println!("Vertices ({}): {:?}", vertices.len(), vertices);
    println!("Indices ({}): {:?}", indices.len(), indices);
}
