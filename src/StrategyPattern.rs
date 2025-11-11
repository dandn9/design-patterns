trait FlyBehavior {
    fn fly(&self) -> ();
}

struct Duck;
struct Pigeon;

impl FlyBehavior for Duck {
    fn fly(&self) -> () {
        println!("Quack");
    }
}

impl FlyBehavior for Pigeon {
    fn fly(&self) -> () {
        println!("Caw caw");
    }
}

struct Bird {
    fly_strategy: Box<dyn FlyBehavior>,
}

impl Bird {
    fn new(strategy: Box<dyn FlyBehavior>) -> Self {
        Bird {
            fly_strategy: strategy,
        }
    }
    fn perform_fly(&self) {
        self.fly_strategy.fly();
    }
    fn set_strategy(&mut self, strategy: Box<dyn FlyBehavior>) {
        self.fly_strategy = strategy;
    }
}

pub fn strategy_pattern() {
    let mut bird = Bird::new(Box::new(Duck));

    bird.perform_fly();

    bird.set_strategy(Box::new(Pigeon));

    bird.perform_fly()
}

mod test {
    use crate::StrategyPattern::strategy_pattern;

    #[test]
    fn it_should_print_the_correct_stuff() {
        strategy_pattern();
    }
}
