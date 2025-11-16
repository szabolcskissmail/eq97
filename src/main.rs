fn main() {
    let eq1_left = |x: f64, y: f64| -> f64 { (2.0 * (3.0 * x - y)) / 5.0 };
    let eq1_right = |x: f64, y: f64| -> f64 { (3.0 * y - 10.0 * x) / 3.0 + 2.0 * x + 1.0 };

    let eq2_left = |x: f64, y: f64| -> f64 { (4.0 * x - 3.0 * y) / 3.0 + (8.0 * x - 3.0 * y) / 2.0 };
    let eq2_right = |x: f64, y: f64| -> f64 { y + 1.0 };

    // Build the linear system A * [x,y] = b from the algebraic derivation:
    // (1) 38x - 21y = 15
    // (2) 32x - 21y = 6
    // Solve using nalgebra
    use nalgebra::{Matrix2, Vector2};

    let a = Matrix2::new(38.0, -21.0,
                         32.0, -21.0);
    let b = Vector2::new(15.0, 6.0);

    let sol = a.lu().solve(&b).expect("matrix is singular");
    let x = sol[0];
    let y = sol[1];

    println!("Solution (nalgebra): x = {}, y = {}", x, y);
    println!("eq1_left(x,y)  = {}", eq1_left(x, y));
    println!("eq1_right(x,y) = {}", eq1_right(x, y));
    println!("eq2_left(x,y)  = {}", eq2_left(x, y));
    println!("eq2_right(x,y) = {}", eq2_right(x, y));
}
