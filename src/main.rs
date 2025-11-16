fn main() {
    let eq1_left = |x: f64, y: f64| -> f64 { (2.0 * (3.0 * x - y)) / 5.0 };
    let eq1_right = |x: f64, y: f64| -> f64 { (3.0 * y - 10.0 * x) / 3.0 + 2.0 * x + 1.0 };

    let eq2_left = |x: f64, y: f64| -> f64 { (4.0 * x - 3.0 * y) / 3.0 + (8.0 * x - 3.0 * y) / 2.0 };
    let eq2_right = |x: f64, y: f64| -> f64 { y + 1.0 };

    // Solve the linear system:
    //  (1)  (2*(3x - y))/5 = (3y - 10x)/3 + 2x + 1  -> 38x - 21y = 15
    //  (2)  (4x-3y)/3 + (8x-3y)/2 = y + 1           -> 32x - 21y = 6
    // Subtracting gives 6x = 9 -> x = 3/2, then y = 2
    let x: f64 = 3.0 / 2.0;
    let y: f64 = 2.0;

    println!("Solution: x = {}, y = {}", x, y);
    println!("eq1_left(x,y)  = {}", eq1_left(x, y));
    println!("eq1_right(x,y) = {}", eq1_right(x, y));
    println!("eq2_left(x,y)  = {}", eq2_left(x, y));
    println!("eq2_right(x,y) = {}", eq2_right(x, y));
}
