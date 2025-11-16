use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&mut self, new_state: u32);
    fn get_id(&self) -> u32;
}

trait Subject {
    fn subscribe(&mut self, obs: Rc<RefCell<dyn Observer>>);
    fn remove(&mut self, obs: u32);
    fn update(&mut self, new_state: u32);
}

struct SubjectImpl {
    obs: Vec<Rc<RefCell<dyn Observer>>>,
    state: u32,
}

struct ObserverImpl {
    id: u32,
    current_state: u32,
}
impl Observer for ObserverImpl {
    fn update(&mut self, new_state: u32) {
        self.current_state = new_state;
        println!("[{}] New state! {new_state}", self.id);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Subject for SubjectImpl {
    fn subscribe(&mut self, obs: Rc<RefCell<dyn Observer>>) {
        self.obs.push(obs);
    }

    fn remove(&mut self, obs_id: u32) {
        if let Some(index) = self.obs.iter().position(|o| o.borrow().get_id() == obs_id) {
            self.obs.remove(index);
        }
    }

    fn update(&mut self, new_state: u32) {
        self.state = new_state;
        for obs in self.obs.iter_mut() {
            obs.borrow_mut().update(self.state);
        }
    }
}

fn observer_pattern() {
    let initial_state = 0;
    let mut subject = SubjectImpl {
        obs: vec![],
        state: initial_state,
    };

    let obs1 = Rc::new(RefCell::new(ObserverImpl {
        id: 1,
        current_state: initial_state,
    }));
    let obs2 = Rc::new(RefCell::new(ObserverImpl {
        id: 2,
        current_state: initial_state,
    }));

    subject.update(1);
    subject.subscribe(obs1.clone());
    subject.update(2);
    subject.subscribe(obs2.clone());
    subject.update(3);

    subject.remove(1);
    subject.update(4);
}

mod test {
    use crate::ObserverPattern::observer_pattern;

    #[test]
    fn it_should_run() {
        observer_pattern();
    }
}
