// Meters new type
struct Meters(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn value(&self) -> f64 {
        self.0
    }

    fn add(&self, other: &Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

fn main() {
    let length1 = Meters::new(5.0);
    let length2 = Meters::new(3.0);

    println!("Length 1: {} meters", length1.value());
    println!("Length 2: {} meters", length2.value());

    let total_length = length1.add(&length2);
    println!("Total length: {} meters", total_length.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let length = Meters::new(5.0);
        assert_eq!(length.value(), 5.0);
    }

    #[test]
    fn test_add() {
        let length1 = Meters::new(5.0);
        let length2 = Meters::new(3.0);
        let total_length = length1.add(&length2);
        assert_eq!(total_length.value(), 8.0);
    }
}
