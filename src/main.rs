extern crate std;
use std::mem;
use std::io;
use std::time::Instant;


fn main() {
//unsafe - позволяет выполнять операции, которые не подпадают
//под контроль безопасности, предоставляемый Rust
unsafe {
let start = Instant::now();

//Ввод количества векторов
let mut length: String = String::new();
println!("Enter the length vectors: {length}");

io::stdin()
.read_line(&mut length)
.expect("Failed to read line");


let parsed_length: i64 = match length.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input. Please enter a valid integer.");
        return; 
    }
};

let end = start.elapsed();
    println!("Время выполнения: {:?}", end);

let size_of_element = std::mem::size_of::<f64>();
let total_size = parsed_length * size_of_element as i64;

let vector_a: *mut f64 = libc::malloc(total_size as libc::size_t) as *mut f64;
let vector_b: *mut f64 = libc::malloc(total_size as libc::size_t) as *mut f64;

//Ввод для значений в вектор
let mut num_a: f64 = 0.0;
let mut num_b: f64 = 0.0;

for i in 0..parsed_length {
    println!("Enter the vectorsA: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    num_a = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return; // Выход из программы при ошибке ввода
        }
    };
    // Используйте индексацию указателя для установки значения
    (*vector_a.offset(i as isize)) = num_a;  
}

for i in 0..parsed_length {
    println!("Enter the vectorsB: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    num_b = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return; // Выход из программы при ошибке ввода
        }
    };
    // Используйте индексацию указателя для установки значения
    (*vector_b.offset(i as isize)) = num_b;  
}

let end = start.elapsed();
    println!("Время выполнения: {:?}", end);
////////////////////////////////

let mut result: f64 = 0.0;
for i in 0..parsed_length {
result += (*vector_a.offset(i as isize)) * (*vector_b.offset(i as isize));
}
    println!("Scalar vector result: {}", result);
    libc::free(vector_a as *mut libc::c_void);
    libc::free(vector_b as *mut libc::c_void);

    // Код, который нужно измерить
    let end = start.elapsed();
    println!("Время выполнения: {:?}", end);
}
}