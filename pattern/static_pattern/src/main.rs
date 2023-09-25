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
    let winter_tyre = WinterTyre;
    let automatic_gear = AutomaticGearShift;

    println!("Winter Tyre: ${}", winter_tyre.cost());
    println!("Automatic Gear Shift: ${}", automatic_gear.cost());

    let car_with_winter_tyre = UpgradeCar(winter_tyre);
    let car_with_automatic_gear = UpgradeCar(automatic_gear);

    println!("Car with winter tyre: ${}", car_with_winter_tyre.cost());
    println!(
        "Car with automatic gear shift: ${}",
        car_with_automatic_gear.cost()
    );
}
