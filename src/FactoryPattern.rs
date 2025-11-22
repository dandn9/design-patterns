struct PizzaShop {
    pizza_factory: Box<dyn PizzaFactory>,
}

impl PizzaShop {
    fn order_pizza(&self) -> Pizza {
        self.pizza_factory.create_pizza()
    }
}

#[derive(Debug)]
struct Pizza {
    name: String,
    cheese: bool,
    pepperoni: bool,
}
trait PizzaFactory {
    fn create_pizza(&self) -> Pizza;
}

struct MargheritaFactory;

impl PizzaFactory for MargheritaFactory {
    fn create_pizza(&self) -> Pizza {
        Pizza {
            name: String::from("Margherita"),
            cheese: true,
            pepperoni: false,
        }
    }
}
struct PepperoniFactory;

impl PizzaFactory for PepperoniFactory {
    fn create_pizza(&self) -> Pizza {
        Pizza {
            name: String::from("Pepperoni"),
            cheese: true,
            pepperoni: true,
        }
    }
}

fn factory_pattern() {
    let mut pizza_shop = PizzaShop {
        pizza_factory: Box::new(MargheritaFactory),
    };
    dbg!(pizza_shop.order_pizza());

    pizza_shop.pizza_factory = Box::new(PepperoniFactory);

    dbg!(pizza_shop.order_pizza());
}

mod test {
    use crate::FactoryPattern::*;

    #[test]
    fn it_should_run() {
        factory_pattern();
    }
}
