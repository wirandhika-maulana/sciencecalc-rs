use sciencecalc_rs::math::algebra::{factorial, combination, permutation};
use sciencecalc_rs::math::matrix::{add_matrix, multiply_matrix, determinant_2x2};
use sciencecalc_rs::math::quadratic::solve_quadratic;
use sciencecalc_rs::math::linear::solve_linear_1x1;
use sciencecalc_rs::math::linear::solve_linear_2x2;
use sciencecalc_rs::math::trigonometry::{sin, cos, tan};
use sciencecalc_rs::math::statistics::{mean, median, mode};

fn main() {
    println!("=== Algebra ===");
    println!("Factorial of 5: {}", factorial(5));
    println!("Combination 5C2: {}", combination(5, 2));
    println!("Permutation 5P2: {}", permutation(5, 2));

    println!("\n=== Matrix ===");
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![4.0, 3.0], vec![2.0, 1.0]];
    println!("Matrix addition: {:?}", add_matrix(&a, &b));
    println!("Matrix multiplication: {:?}", multiply_matrix(&a, &b));
    println!("Determinant (2x2): {}", determinant_2x2(&[[1.0, 2.0], [3.0, 4.0]]));

    println!("\n=== Quadratic ===");
    match solve_quadratic(1.0, -3.0, 2.0) {
        Some((x1, x2)) => println!("Roots: x1 = {}, x2 = {}", x1, x2),
        None => println!("No real roots."),
    }

    println!("\n=== Linear Equation 1x1 ===");
    match solve_linear_1x1(2.0, 3.0, 11.0) {
    Some(x) => println!("x = {}", x),
    None => println!("No unique solution."),
    }

    println!("\n=== Linear Equation 2x2 ===");
    match solve_linear_2x2(2.0, 3.0, 8.0, 1.0, -4.0, -2.0) {
        Some((x, y)) => println!("x = {}, y = {}", x, y),
        None => println!("No unique solution."),
    }

    println!("\n=== Trigonometry ===");
    println!("sin(30°): {}", sin(30.0));
    println!("cos(60°): {}", cos(60.0));
    println!("tan(45°): {}", tan(45.0));

    println!("\n=== Statistics ===");
    let mut data = vec![1.0, 2.0, 2.0, 3.0, 4.0];
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&mut data));
    println!("Mode: {:?}", mode(&data));
}
