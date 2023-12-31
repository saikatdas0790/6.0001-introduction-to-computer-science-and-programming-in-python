use crate::trigger::Trigger;

pub trait PhraseTrigger: Trigger {
    fn new(phrase: &str) -> Self;
    fn is_phrase_in(&self, text: &str) -> bool;
}
