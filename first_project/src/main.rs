fn main() {
    println!("Let's Get Rusty!");
    println!("1 + 2 = {}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
