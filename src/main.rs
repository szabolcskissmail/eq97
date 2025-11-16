fn main() {

    let eq1_left = |x: f64, y: f64| -> f64 {
        (2.0 * (3.0 * x - y)) / 5.0
    };

    let eq1_right = |x: f64, y: f64| -> f64 {
        (3.0 * y - 10.0 * x) / 3.0 + 2.0 * x + 1.0
    };


    let eq2_left = |x: f64, y: f64| -> f64 {
        (4.0 * x - 3.0 * y) / 3.0 + (8.0 * x - 3.0 * y) / 2.0
    };

    let eq2_right = |x: f64, y: f64| -> f64 {
        y + 1.0
    };


    let v = eq1_left(41,12);
    println!("eq1_left = {}", v);
}
