fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (a, b) = pair;
    (b, a)
}

fn transpose(m: Matrix) -> Matrix{
    let Matrix(a, b, c, d) = m;
    return Matrix(d, c, b, a);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1, 2), ("Sam", 0.5f64));
    println!("tuple of tuples {:?}", tuple_of_tuples);

    // Tuples with more than 12 elements can't be printed.

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));

    println!("Tuple of one: {:?}", ("Hi",));
    
    let tuple = (1, 2, 3);
    let (a, b, c) = tuple; // destructuring.

    let matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("{:?}", matrix);
    println!("{:?}", transpose(matrix));
}