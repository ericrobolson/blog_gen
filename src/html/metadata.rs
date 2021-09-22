use super::Html;

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    pub author: String,
    pub keywords: Vec<String>,
    pub summary: String,
}

struct Kv<'a> {
    k: &'a str,
    v: &'a str,
}

struct Meta<'a> {
    kv: Vec<Kv<'a>>,
}

impl Html for Metadata {
    fn to_html(&self) -> String {
        // Build up all metadata tags, then spit out the HTML
        // https://www.w3schools.com/tags/tag_meta.asp
        let keywords = self.keywords.join(", ");
        let items = vec![
            Meta {
                kv: vec![Kv {
                    k: "charset",
                    v: "UTF-8",
                }],
            },
            Meta {
                kv: vec![
                    Kv {
                        k: "name",
                        v: "description",
                    },
                    Kv {
                        k: "content",
                        v: &self.summary,
                    },
                ],
            },
            Meta {
                kv: vec![
                    Kv {
                        k: "name",
                        v: "keywords",
                    },
                    Kv {
                        k: "content",
                        v: &keywords,
                    },
                ],
            },
            Meta {
                kv: vec![
                    Kv {
                        k: "name",
                        v: "author",
                    },
                    Kv {
                        k: "content",
                        v: &self.author,
                    },
                ],
            },
            Meta {
                kv: vec![
                    Kv {
                        k: "name",
                        v: "viewport",
                    },
                    Kv {
                        k: "content",
                        v: "width=device-width, initial-scale=1.0",
                    },
                ],
            },
        ];

        items
            .iter()
            .map(|i| {
                let kvs =
                    i.kv.iter()
                        .map(|kv| format!("{}=\"{}\"", kv.k, kv.v))
                        .collect::<Vec<String>>()
                        .join(" ");

                format!("<meta {}>", kvs)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
