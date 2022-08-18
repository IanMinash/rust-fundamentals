use std::io;

fn main() {
    let mut input = String::new(); //Is stored in a heap. Its a smart pointer.
    
    /*
     * In this case, the ownership of `input` is moved to the s variable within some_fn
     * Once some_fn goes out of scope, s will be destructured which could possibly lead 
     * to a memory error when trying to use the input variable again. Luckily the Rust
     * compiler prevents this😅 
     */ 
    // own_fn(input); 

    // borrow_fn(&input);

    println!("Enter your weight in kg: ");
    io::stdin().read_line(&mut input).unwrap();
    let mut mars_weight = calculate_weight_on_mars(input.trim().parse::<f32>().unwrap());
    println!("Weight on Mars: {} kg", mars_weight);
    mars_weight = mars_weight * 1000.0; 
    println!("Weight on Mars: {} g", mars_weight);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    (weight/9.81) * 3.711
}

fn own_fn(s: String) {
    println!("{}",s)
}

/**
 * Because some_borrowing_fn gets a reference to a String into the s variable. The 
 * string is not dropped once some_borrowing_fn goes out of scope. This is how 
 * variables can be borrowed in Rust. As with other variables, references are
 * immutbable by default.
 * 
 * You can only have a single mutable borrow or multiple immutable borrows. 
 */
fn borrow_fn(s: &String)  {
    println!("{}", s)
}