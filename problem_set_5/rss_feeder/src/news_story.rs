#![allow(dead_code)]

use std::time::SystemTime;

pub struct NewsStory {
    guid: String,
    title: String,
    description: String,
    link: String,
    pubdate: SystemTime,
}

impl NewsStory {
    pub fn new(
        guid: String,
        title: String,
        description: String,
        link: String,
        pubdate: SystemTime,
    ) -> NewsStory {
        NewsStory {
            guid,
            title,
            description,
            link,
            pubdate,
        }
    }

    pub fn get_guid(&self) -> &String {
        &self.guid
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_link(&self) -> &String {
        &self.link
    }

    pub fn get_pubdate(&self) -> &SystemTime {
        &self.pubdate
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_news_story_constructor() {
        let _story = NewsStory::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            SystemTime::now(),
        );
    }

    #[test]
    fn test_news_story_get_guid() {
        let story = NewsStory::new(
            String::from("test guid"),
            String::from("test title"),
            String::from("test description"),
            String::from("test link"),
            SystemTime::now(),
        );
        assert_eq!(story.get_guid(), "test guid");
    }

    #[test]
    fn test_news_story_get_title() {
        let story = NewsStory::new(
            String::from("test guid"),
            String::from("test title"),
            String::from("test description"),
            String::from("test link"),
            SystemTime::now(),
        );
        assert_eq!(story.get_title(), "test title");
    }

    #[test]
    fn test_news_story_get_description() {
        let story = NewsStory::new(
            String::from("test guid"),
            String::from("test title"),
            String::from("test description"),
            String::from("test link"),
            SystemTime::now(),
        );
        assert_eq!(story.get_description(), "test description");
    }

    #[test]
    fn test_news_story_get_link() {
        let story = NewsStory::new(
            String::from("test guid"),
            String::from("test title"),
            String::from("test description"),
            String::from("test link"),
            SystemTime::now(),
        );
        assert_eq!(story.get_link(), "test link");
    }

    #[test]
    fn test_news_story_get_time() {
        let story = NewsStory::new(
            String::from("test guid"),
            String::from("test title"),
            String::from("test description"),
            String::from("test link"),
            SystemTime::now(),
        );
        assert!(story.get_pubdate().elapsed().unwrap() < Duration::from_secs(1));
    }
}
