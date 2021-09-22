use crate::{
    parser::{ContentIr, LinkType, Page},
    Context, Item,
};
use std::sync::Arc;

pub fn build(html_filename: String, context: &Arc<Context>) -> Page {
    let projects = ContentIr::section(
        "Projects".into(),
        vec![ContentIr::Text {
            text: "A collection of various projects I've worked on over the years.".into(),
        }],
    );

    let page_contents = vec![
        ContentIr::title("Projects".into(), "projects".into()),
        projects,
        map_projects(context),
    ];

    Page {
        contents: Item {
            location: None,
            item: ContentIr::Div {
                contents: page_contents,
            },
        },
        file_name: html_filename,
        keywords: map_keywords(context),
        summary: "A collection of projects I've worked on over the years.".into(),
        title: "Projects".into(),
    }
}

fn map_projects(context: &Arc<Context>) -> ContentIr {
    let projects = context
        .config
        .projects
        .iter()
        .map(|p| {
            ContentIr::section(
                p.title.clone(),
                vec![
                    ContentIr::text(p.description.clone()),
                    ContentIr::Link {
                        description: "Check it out".into(),
                        url: p.url.clone(),
                        link_type: LinkType::Regular,
                    },
                ],
            )
        })
        .collect();

    ContentIr::Div { contents: projects }
}

fn map_keywords(context: &Arc<Context>) -> Vec<String> {
    let mut keywords = vec![
        "github".into(),
        "collection".into(),
        "various".into(),
        "projects".into(),
    ];

    context.config.projects.iter().for_each(|p| {
        keywords.append(
            &mut p
                .description
                .split_ascii_whitespace()
                .map(|c| c.to_lowercase().replace(".", "").to_string())
                .filter(|c| {
                    c != "and" && c != "that" && c != "the" && c != "a" && c != "for" && c != "in"
                })
                .collect(),
        )
    });

    keywords.dedup();

    keywords
}
