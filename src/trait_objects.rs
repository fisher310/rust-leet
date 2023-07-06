use std::fmt::Debug;

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        // 特征对象是由 dyn Area 表示的，意味着它是指向Area特征某些实现的指针。
        let shapes: Vec<&dyn Area> = vec![&Square(3_f32), &Rectangle(4_f32, 2_f32)];
        for s in shapes {
            println!("{:?}", s.get_area());
        }
    }
}
