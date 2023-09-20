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

trait Topping: Coffee {
    fn description(&self) -> String;
}

struct Milk<T: Coffee>(T);

impl<T: Coffee> Coffee for Milk<T> {
    fn cost(&self) -> f64 {
        self.0.cost()
    }
}

impl<T: Coffee> Topping for Milk<T> {
    fn description(&self) -> String {
        format!("Milk, {}", self.0.cost())
    }
}

fn main() {
    let espresso = Espresso;
    let latte = Latte;

    println!("Espresso: ${}", espresso.cost());
    println!("Latte: ${}", latte.cost());

    let coffee_with_milk = Milk(espresso);
    let coffee_with_double_milk = Milk(Milk(latte));

    println!(
        "Coffee with Milk: ${}, Description: {}",
        coffee_with_milk.cost(),
        coffee_with_milk.description()
    );
    println!(
        "Coffee with Double Milk: ${}, Description: {}",
        coffee_with_double_milk.cost(),
        coffee_with_double_milk.description()
    );
}
