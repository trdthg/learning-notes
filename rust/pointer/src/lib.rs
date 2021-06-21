


// RefCell与内部可变性模式
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
} impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {messenger, value: 0, max}
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max > 0.75{
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    struct MockMessanger {
        sent_messages: RefCell<Vec<String>>,
    }
    use std::cell::RefCell;
    impl MockMessanger {
        fn new() -> MockMessanger {
            MockMessanger {sent_messages: RefCell::new(vec![])}
        }
    }
    impl Messenger for MockMessanger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
    let mock_messanger = MockMessanger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);
    limit_tracker.set_value(80);
    assert_eq!(mock_messanger.sent_messages.borrow().len(), 1); 
    println!("{:?}", mock_messanger.sent_messages);
    
}
