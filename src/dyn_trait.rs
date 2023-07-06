use std::fmt::Display;

fn show_me(item: &dyn Display) {
    println!("{}", item);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        show_me(&"hello trait object");
    }
}
