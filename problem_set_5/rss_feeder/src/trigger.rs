use crate::news_story::NewsStory;

pub trait Trigger {
    fn evaluate(&self, story: &NewsStory) -> bool;
}
