mod about_me_page;
mod catalog;
mod index;
mod projects_page;

use crate::{
    parser::{ContentIr, Css, Img, IntermediateRepresentation, LinkType, Md, Page},
    Context, Item, Location, Res,
};
use std::sync::Arc;

pub(crate) struct BuildContext {
    pub css: Vec<(Option<Location>, Css)>,
    pub img: Vec<(Option<Location>, Img)>,
    pub md: Vec<(Option<Location>, Md)>,
    pub page: Vec<(Option<Location>, Page)>,
}

pub fn execute(
    context: Arc<Context>,
    mut ir: Vec<Item<IntermediateRepresentation>>,
) -> Vec<Res<IntermediateRepresentation>> {
    let build_context = {
        let mut build_context = BuildContext {
            css: vec![],
            img: vec![],
            md: vec![],
            page: vec![],
        };

        while ir.len() > 0 {
            let i = ir.remove(0);
            let location = i.location;

            match i.item {
                IntermediateRepresentation::Css(item) => build_context.css.push((location, item)),
                IntermediateRepresentation::Img(item) => build_context.img.push((location, item)),
                IntermediateRepresentation::Md(item) => build_context.md.push((location, item)),
                IntermediateRepresentation::Page(item) => build_context.page.push((location, item)),
            }
        }

        // Preprocess all file names for css
        for (_, css) in build_context.css.iter_mut() {
            css.file_name = css.file_name.to_lowercase();
        }

        // Preprocess all file names for imgs
        for (_, img) in build_context.img.iter_mut() {
            img.file_name = img.file_name.to_lowercase();
        }

        Arc::new(build_context)
    };

    // Build out the MDs
    let mds = build_mds(&build_context, &context);

    // Build the final IR
    let mut ir = vec![];

    for (location, css) in build_context.css.iter() {
        ir.push(Res::Ok(Item {
            location: location.clone(),
            item: IntermediateRepresentation::Css(css.clone()),
        }));
    }

    for (location, img) in build_context.img.iter() {
        ir.push(Res::Ok(Item {
            location: location.clone(),
            item: IntermediateRepresentation::Img(img.clone()),
        }));
    }

    // Build the mds
    for res in mds {
        let res = match res {
            Res::Ok(i) => Res::Ok(Item {
                item: IntermediateRepresentation::Md(i.item),
                location: i.location,
            }),
            Res::Warn { item, msg } => Res::Warn {
                item: Item {
                    location: item.location,
                    item: IntermediateRepresentation::Md(item.item),
                },
                msg,
            },
            Res::Warnings { item, msgs } => Res::Warnings {
                item: Item {
                    location: item.location,
                    item: IntermediateRepresentation::Md(item.item),
                },
                msgs,
            },
            Res::Error { location, msg } => Res::Error { location, msg },
        };

        ir.push(res);
    }

    // Build the standalone pages
    for page in build_standalone_pages(
        context.links.about_me_link.clone(),
        context.links.catalog_link.clone(),
        context.links.index_link.clone(),
        context.links.projects_link.clone(),
        &build_context,
        context.clone(),
    ) {
        let res = match page {
            Res::Ok(i) => Res::Ok(Item {
                item: IntermediateRepresentation::Page(i.item),
                location: i.location,
            }),
            Res::Warn { item, msg } => Res::Warn {
                item: Item {
                    location: item.location,
                    item: IntermediateRepresentation::Page(item.item),
                },
                msg,
            },
            Res::Warnings { item, msgs } => Res::Warnings {
                item: Item {
                    location: item.location,
                    item: IntermediateRepresentation::Page(item.item),
                },
                msgs,
            },
            Res::Error { location, msg } => Res::Error { location, msg },
        };

        ir.push(res);
    }

    ir
}

fn build_standalone_pages(
    about_me_link: String,
    catalog_link: String,
    index_link: String,
    projects_link: String,
    build_context: &Arc<BuildContext>,
    context: Arc<Context>,
) -> Vec<Res<Page>> {
    let about_me = Res::Ok(Item {
        item: about_me_page::build(about_me_link, &context),
        location: None,
    });

    let catalog = Res::Ok(Item {
        item: catalog::build(catalog_link, build_context),
        location: None,
    });

    let projects = Res::Ok(Item {
        item: projects_page::build(projects_link, &context),
        location: None,
    });

    let index = Res::Ok(Item {
        item: index::build(index_link, &build_context, &context),
        location: None,
    });

    vec![index, catalog, projects, about_me]
}

#[allow(unreachable_code)]
fn build_mds(build_context: &Arc<BuildContext>, context: &Arc<Context>) -> Vec<Res<Md>> {
    #[cfg(feature = "multithread")]
    {
        use rayon::prelude::*;
        return build_context
            .md
            .par_iter()
            .enumerate()
            .map(|(idx, (location, md))| {
                build_md(
                    idx,
                    location.clone(),
                    md.clone(),
                    build_context.clone(),
                    context.clone(),
                )
            })
            .collect();
    }

    build_context
        .md
        .iter()
        .enumerate()
        .map(|(idx, (location, md))| {
            build_md(
                idx,
                location.clone(),
                md.clone(),
                build_context.clone(),
                context.clone(),
            )
        })
        .collect()
}

fn build_md(
    index: usize,
    location: Option<Location>,
    mut md: Md,
    build_context: Arc<BuildContext>,
    context: Arc<Context>,
) -> Res<Md> {
    // Build out the links
    if build_context.md.len() > 0 {
        // Previous
        if index != 0 {
            md.navigation
                .previous
                .push(build_context.md[index - 1].1.html_file_name.clone());
        }

        // Next
        if index < build_context.md.len() - 1 {
            md.navigation
                .next
                .push(build_context.md[index + 1].1.html_file_name.clone());
        }
    }

    let mut warnings = vec![];

    match validate_md(&md.contents.item, &md.contents.location, &build_context) {
        Res::Ok(_) => {}
        Res::Warn { item, msg } => {
            warnings.push((msg, item.location.clone()));
        }
        Res::Warnings { mut msgs, .. } => {
            warnings.append(&mut msgs);
        }
        Res::Error { location, msg } => return Res::Error { location, msg },
    };

    // Build out navigation
    let md = {
        let navigation = ContentIr::Navigation {
            links: context.links.clone(),
            next: match md.navigation.next.first() {
                Some(s) => Some(s.clone()),
                None => None,
            },
            previous: match md.navigation.previous.last() {
                Some(s) => Some(s.clone()),
                None => None,
            },
        };

        md.contents = Item {
            item: ContentIr::Div {
                contents: vec![navigation, md.contents.item],
            },
            location: md.contents.location,
        };
        md
    };

    let item = Item { location, item: md };
    if warnings.is_empty() {
        Res::Ok(item)
    } else if warnings.len() == 1 {
        Res::Warn {
            item,
            msg: warnings[0].0.clone(),
        }
    } else {
        Res::Warnings {
            item,
            msgs: warnings,
        }
    }
}

fn validate_md(
    content: &ContentIr,
    location: &Option<Location>,
    build_context: &Arc<BuildContext>,
) -> Res<()> {
    let mut warnings: Vec<(String, Option<Location>)> = vec![];
    let mut errors: Vec<Res<()>> = vec![];

    let mut map_results = |res: Res<()>| match res {
        Res::Ok(_) => {}
        Res::Warn { item, msg } => warnings.push((msg, item.location)),
        Res::Warnings { mut msgs, .. } => warnings.append(&mut msgs),
        Res::Error { location, msg } => errors.push(Res::Error::<()> { location, msg }),
    };

    match &content {
        ContentIr::Block { contents, .. } => {
            map_results(validate_md(contents.as_ref(), location, build_context))
        }
        ContentIr::Card(content) => content
            .iter()
            .for_each(|c| map_results(validate_md(c, location, build_context))),
        ContentIr::Div { contents } => contents
            .iter()
            .for_each(|c| map_results(validate_md(c, location, build_context))),
        ContentIr::Header { .. } => {}
        ContentIr::InlineCode(_) => {}
        ContentIr::Link { url, link_type, .. } => {
            // validate the various links
            match link_type {
                LinkType::Iframe => {
                    // not validating external links
                }
                LinkType::LocalImage => {
                    // Only validate local images
                    const ASSETS_FOLDER: &'static str = "assets/";
                    if url.starts_with(ASSETS_FOLDER) {
                        let mut found = false;
                        let raw_url = url.replace(ASSETS_FOLDER, "");
                        for img in build_context.img.iter() {
                            if raw_url == img.1.file_name {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            errors.push(Res::Error {
                                location: location.clone(),
                                msg: format!("Unlinked image with raw url: {:?}!", raw_url),
                            })
                        }
                    }
                }
                LinkType::Regular => {
                    // not validating external links
                }
                LinkType::Youtube => {
                    // not validating external links
                }
            }
        }
        ContentIr::Metadata(_) => {}
        ContentIr::Navigation { .. } => {}
        ContentIr::OrderedList { items, .. } => items
            .iter()
            .for_each(|c| map_results(validate_md(c, location, build_context))),
        ContentIr::Paragraph { children } => children
            .iter()
            .for_each(|c| map_results(validate_md(c, location, build_context))),
        ContentIr::Text { .. } => {}
        ContentIr::Unparsed { .. } => todo!(),
        ContentIr::UnorderedList { items } => items
            .iter()
            .for_each(|c| map_results(validate_md(c, location, build_context))),
    };

    if errors.len() > 0 {
        return errors.remove(0);
    }

    let item = Item {
        location: location.clone(),
        item: (),
    };

    if warnings.is_empty() {
        Res::Ok(item)
    } else if warnings.len() == 1 {
        Res::Warn {
            item,
            msg: warnings[0].0.clone(),
        }
    } else {
        Res::Warnings {
            item,
            msgs: warnings,
        }
    }
}
