use crate::trigger::Trigger;

pub trait PhraseTrigger: Trigger {
    fn new(phrase: &str) -> Self;
    fn is_phrase_in(&self, text: &str) -> bool;
}

pub struct PhraseTriggerImpl {
    phrase: String,
}

impl PhraseTriggerImpl {
    pub fn new(phrase: &str) -> Self {
        PhraseTriggerImpl {
            phrase: phrase.to_ascii_lowercase(),
        }
    }

    pub fn is_phrase_in(&self, text: &str) -> bool {
        text.to_ascii_lowercase().contains(&self.phrase)
    }
}
