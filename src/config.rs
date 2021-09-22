use crate::theme::Theme;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub about_me: String,
    pub about_me_keywords: Vec<String>,
    pub about_me_summary: String,
    pub author: String,
    pub github: Option<String>,
    pub index_keywords: Vec<String>,
    pub index_title: String,
    pub linked_in: Option<String>,
    pub projects: Vec<Project>,
    #[serde(default)]
    pub theme: Theme,
    pub twitter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub url: String,
}

impl Config {
    pub fn example_config() -> Self {
        Self {
            about_me: "Required: A blurb about you!".into(),
            about_me_keywords: vec!["Required: About Me keywords".into()],
            about_me_summary: "Required: About Me summary".into(),
            author: "Required: Your name goes here".into(),
            github: Some("Optional github profile".into()),
            index_keywords: vec!["Required: Index keywords".into()],
            index_title: "Required: Some Index Title".into(),
            linked_in: Some("Optional linked in profile".into()),
            projects: vec![Project {
                title: "Example Title".into(),
                description: "Example Description".into(),
                url: "ExampleUrl".into(),
            }],
            theme: Theme::default(),
            twitter: Some("Optional twitter".into()),
        }
    }
}
