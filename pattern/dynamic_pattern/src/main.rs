trait Car {
    fn cost(&self) -> f64;
}

struct WinterTyre;

impl Car for WinterTyre {
    fn cost(&self) -> f64 {
        100.0
    }
}

struct AutomaticGearShift;

impl Car for AutomaticGearShift {
    fn cost(&self) -> f64 {
        450.0
    }
}

struct UpgradeCar<T: Car>(T);

impl<T: Car> Car for UpgradeCar<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 59.99 // Cost of upgrade
    }
}

fn main() {
    let espresso = WinterTyre;
    let latte = AutomaticGearShift;

    println!("Winter Tyre: ${}", espresso.cost());
    println!("Automatic Gear Shift: ${}", latte.cost());

    let coffee_with_milk = UpgradeCar(espresso);
    let coffee_with_double_milk = UpgradeCar(UpgradeCar(latte));

    println!("Car with winter tyre: ${}", coffee_with_milk.cost());
    println!(
        "Car with automatic gear shift: ${}",
        coffee_with_double_milk.cost()
    );
}
