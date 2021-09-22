use super::BuildContext;
use crate::{
    parser::{ContentIr, LinkType, Page},
    Context, Item,
};
use std::sync::Arc;

pub(crate) fn build(
    html_filename: String,
    build_context: &Arc<BuildContext>,
    context: &Arc<Context>,
) -> Page {
    let file_name = html_filename;

    let mut page_contents = vec![ContentIr::title("Home".into(), "indexTitle".into())];

    let mut content = vec![
        ContentIr::section("Recent Posts".into(), recent_posts(build_context)),
        last_post(build_context),
        // TODO: Series calculations
    ];

    page_contents.append(&mut content);

    Page {
        contents: Item {
            location: None,
            item: ContentIr::Div {
                contents: page_contents,
            },
        },
        file_name,
        keywords: context.config.index_keywords.clone(),
        summary: "Home page".into(),
        title: context.config.index_title.clone(),
    }
}

fn last_post(build_context: &Arc<BuildContext>) -> ContentIr {
    let (_, md) = build_context.md.iter().last().unwrap();

    md.contents.item.clone()
}

const RECENT_POST_COUNT: usize = 7;

fn recent_posts(build_context: &Arc<BuildContext>) -> Vec<ContentIr> {
    vec![ContentIr::UnorderedList {
        items: build_context
            .md
            .iter()
            .rev()
            .take(RECENT_POST_COUNT)
            .map(|(_, md)| ContentIr::Div {
                contents: vec![
                    ContentIr::Link {
                        description: format!("{}: {}", md.date, md.display_title),
                        url: md.html_file_name.to_str().into(),
                        link_type: LinkType::Regular,
                    },
                    ContentIr::text(format!(" - {}", md.summary)),
                ],
            })
            .collect(),
    }]
}
