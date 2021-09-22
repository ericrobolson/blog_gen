use crate::{
    parser::{ContentIr, LinkType, Page},
    Context, Item,
};
use std::sync::Arc;

pub fn build(html_filename: String, context: &Arc<Context>) -> Page {
    let about_me = ContentIr::section(
        "Who I am".into(),
        vec![ContentIr::Text {
            text: context.config.about_me.clone(),
        }],
    );

    let mut page_contents = vec![
        ContentIr::title("About Me".into(), "aboutMe".into()),
        about_me,
    ];

    let mut important_links = vec![];

    if let Some(github_url) = context.config.github.clone() {
        important_links.push(ContentIr::Link {
            description: "Visit my Github".into(),
            link_type: LinkType::Regular,
            url: github_url,
        });
    }

    if let Some(linkedin_url) = context.config.linked_in.clone() {
        important_links.push(ContentIr::Link {
            description: "Visit my LinkedIn".into(),
            link_type: LinkType::Regular,
            url: linkedin_url,
        });
    }

    if let Some(twitter_url) = context.config.twitter.clone() {
        important_links.push(ContentIr::Link {
            description: "Visit my Twitter".into(),
            link_type: LinkType::Regular,
            url: twitter_url,
        });
    }

    if important_links.is_empty() == false {
        page_contents.push(ContentIr::section(
            "Social Media".into(),
            vec![ContentIr::UnorderedList {
                items: important_links,
            }],
        ));
    }

    Page {
        contents: Item {
            location: None,
            item: ContentIr::Div {
                contents: page_contents,
            },
        },
        file_name: html_filename,
        keywords: context.config.about_me_keywords.clone(),
        summary: context.config.about_me_summary.clone(),
        title: "About Me".into(),
    }
}
