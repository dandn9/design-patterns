trait Beverage {
    fn cost(&self) -> f32;
}
struct Coffee;
impl Beverage for Coffee {
    fn cost(&self) -> f32 {
        1.5
    }
}

trait BeverageDecorator: Beverage {
    fn get_beverage(&self) -> &dyn Beverage;
}

struct Whip {
    beverage: Box<dyn Beverage>,
}

impl Beverage for Whip {
    fn cost(&self) -> f32 {
        self.get_beverage().cost() + 0.2
    }
}

impl BeverageDecorator for Whip {
    fn get_beverage(&self) -> &dyn Beverage {
        &(*self.beverage)
    }
}
impl Whip {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}

fn decorator_pattern() {
    let coffee = Coffee;
    let whipped_coffee = Whip::new(Box::new(coffee));
    let double_whipped_coffee = Whip::new(Box::new(whipped_coffee));
    println!("{}", double_whipped_coffee.cost());
}

mod test {
    use crate::DecoratorPattern::decorator_pattern;

    #[test]
    fn it_should_run() {
        decorator_pattern();
    }
}
