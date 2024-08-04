fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut sum;

    match n {
        0 => return a,
        1 => return b,
        _ => {
            for _ in 2..=n {
                sum = a + b;
                a = b;
                b = sum;
            }
        }
    }

    b
}

fn main() {
    let n = 45; // 示例：输出第 10 项
    println!("斐波那契数列的第 {} 项是: {}", n, fibonacci_iterative(n));
}
