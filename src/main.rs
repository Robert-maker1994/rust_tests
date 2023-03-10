#[path = "subtract/sub.rs"] mod subtract;
#[path = "add/add.rs"] mod add;
#[path = "add/add_tests.rs"] mod add_tests;
#[path = "subtract/sub_tests.rs"] mod sub_tests;
#[path = "multiply/positive/multiply.rs"] mod multiply;
#[ path = "multiply/positive/multiply_tests.rs"] mod multiply_tests;
// Type the path first and let the ide create the file path mod value has to e the same. as file nae
// # [ path = "is_whole_number/is_whole.rs" ] mod is_whole;

fn main() {
    let sub_num = subtract::sub(3, 6);
    let add_num = add::add(2,5);
    let multiply = multiply::multiply(3, 6);

    println!("subtracted number equals {}", sub_num);
    println!("added number equals {}", add_num);
    println!("multiply equals {}", multiply);
}

