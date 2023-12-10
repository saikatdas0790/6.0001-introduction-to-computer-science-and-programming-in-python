#![allow(dead_code)]

use std::time::SystemTime;

struct NewsStory {
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

trait Trigger {
    fn evaluate(&self, story: &NewsStory) -> bool;
}
