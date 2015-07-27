use std::collections::HashMap;

#[test]
fn state_change()
{
    use std::rc::Rc;
    use std::cell::RefCell;

    struct States;
    impl States
    {
        fn a() -> &'static str { "a" }
        fn b() -> &'static str { "b" }
        fn c() -> &'static str { "c" }
    }

    struct Log { log: String }
    impl Log
    {
        fn append(&mut self, string: &str)
        {
            self.log = self.log.clone() + string;
        }
    }

    let log = Rc::new(RefCell::new(Log { log: String::new() }));

    let mut state = Stator::new(States::a());

    state.add_handler(States::a(), States::b(), |from: &String|
    {
        println!("a -> b");
        log.borrow_mut().append("1");
    });

    state.add_handler(Stator::any_state(), States::b(), |from: &String|
    {
        println!("? -> b");
        log.borrow_mut().append("2");
    });

    state.add_handler(States::a(), Stator::any_state(), |from: &String|
    {
        println!("a -> ?");
        log.borrow_mut().append("3");
    });

    state.add_handler(Stator::any_state(), States::a(), |from: &String|
    {
        println!("? -> a");
        log.borrow_mut().append("4");
    });

    state.enter(States::b());

    assert!(log.borrow_mut().log == "123", "Unexpected log sequence");

    state.enter(States::c());

    assert!(log.borrow_mut().log == "123", "Unexpected log sequence");

    state.enter(States::b());

    assert!(log.borrow_mut().log == "1232", "Unexpected log sequence");

    state.enter(States::a());

    assert!(log.borrow_mut().log == "12324", "Unexpected log sequence");
}

#[derive(Hash, Eq, PartialEq)]
struct StatorHashKey
{
    from: String,
    to: String
}

pub struct Stator<'a>
{
    current_state: String,
    state_handlers: HashMap<StatorHashKey, Vec<Box<FnMut(&String) + 'a>>>
}

impl<'a> Stator<'a>
{
    pub fn any_state() -> &'static str { "_Stator_Internal__Any_State_" }

    pub fn new(state: &str) -> Stator<'a>
    {
        Stator { current_state: state.to_string(), state_handlers: HashMap::new() }
    }

    pub fn add_handler<F: FnMut(&String) + 'a>(&mut self, from: &str, to: &str, f: F)
    {
        let key = StatorHashKey { from: from.to_string(), to: to.to_string() };

        if self.state_handlers.contains_key(&key)
        {
            self.state_handlers.get_mut(&key).unwrap().push(Box::new(f));
        }else
        {
            self.state_handlers.insert(key, vec![Box::new(f)]);
        }
    }

    pub fn enter(&mut self, state: &str)
    {
        {
            let key = StatorHashKey { from: self.current_state.clone(), to: state.to_string() };

            if self.state_handlers.contains_key(&key)
            {
                let handlers = self.state_handlers.get_mut(&key).unwrap();

                for handler in handlers
                {
                    (*handler)(&self.current_state);
                }
            }
        }

        {
            let key = StatorHashKey { from: Stator::any_state().to_string(), to: state.to_string() };

            if self.state_handlers.contains_key(&key)
            {
                let handlers = self.state_handlers.get_mut(&key).unwrap();

                for handler in handlers
                {
                    (*handler)(&self.current_state);
                }
            }
        }

        {
            let key = StatorHashKey { from: self.current_state.clone(), to: Stator::any_state().to_string() };

            if self.state_handlers.contains_key(&key)
            {
                let handlers = self.state_handlers.get_mut(&key).unwrap();

                for handler in handlers
                {
                    (*handler)(&self.current_state);
                }
            }
        }

        {
            let key = StatorHashKey { from: Stator::any_state().to_string(), to: Stator::any_state().to_string() };

            if self.state_handlers.contains_key(&key)
            {
                let handlers = self.state_handlers.get_mut(&key).unwrap();

                for handler in handlers
                {
                    (*handler)(&self.current_state);
                }
            }
        }

        self.current_state = state.to_string();
    }
}