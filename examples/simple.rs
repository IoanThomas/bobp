fn main() {
    let obj_string = match std::fs::read_to_string("examples/data/plane.obj") {
        Ok(obj_string) => obj_string,
        Err(error) => {
            eprintln!("Failed to load OBJ file: {}", error);
            return;
        }
    };

    let (vertices, indices) = match bobp::parse_obj(obj_string) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse OBJ string: {}", error);
            return;
        }
    };

    println!("Vertices ({}): {:?}", vertices.len(), vertices);
    println!("Indices ({}): {:?}", indices.len(), indices);
}
