trait Coffee {
    fn cost(&self) -> f64;
}

struct Espresso;

impl Coffee for Espresso {
    fn cost(&self) -> f64 {
        1.0
    }
}

struct Latte;

impl Coffee for Latte {
    fn cost(&self) -> f64 {
        2.0
    }
}

struct Milk<T: Coffee>(T);

impl<T: Coffee> Coffee for Milk<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 0.5 // Cost of milk
    }
}

fn main() {
    let espresso = Espresso;
    let latte = Latte;

    println!("Espresso: ${}", espresso.cost());
    println!("Latte: ${}", latte.cost());

    let coffee_with_milk = Milk(espresso);
    let coffee_with_double_milk = Milk(Milk(latte));

    println!("Coffee with Milk: ${}", coffee_with_milk.cost());
    println!(
        "Coffee with Double Milk: ${}",
        coffee_with_double_milk.cost()
    );
}
