use super::blogs::Manifest;
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use eyre::eyre;
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
    title: String,
    #[serde(default)]
    tags: Vec<String>,
    layout: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Post {
    pub(crate) filename: String,
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) tags: Vec<String>,
    pub(crate) year: i32,
    pub(crate) show_year: bool,
    pub(crate) month: u32,
    pub(crate) day: u32,
    pub(crate) contents: String,
    pub(crate) url: String,
    pub(crate) published: String,
    pub(crate) updated: String,
}

impl Post {
    pub(crate) fn open(path: &Path, _manifest: &Manifest) -> eyre::Result<Self> {
        // yeah this might blow up, but it won't
        let filename = path.file_name().unwrap().to_str().unwrap();

        // we need to get the metadata out of the url
        let mut split = filename.splitn(4, "-");

        // we do some unwraps because these need to be valid
        let year = split.next().unwrap().parse::<i32>().unwrap();
        let month = split.next().unwrap().parse::<u32>().unwrap();
        let day = split.next().unwrap().parse::<u32>().unwrap();
        let filename = split.next().unwrap().to_string();

        let contents = std::fs::read_to_string(path)?;
        if contents.len() < 5 {
            return Err(eyre!(
                "{path:?} is empty, or too short to have valid front matter"
            ));
        }

        // yaml headers.... we know the first four bytes of each file are "---\n"
        // so we need to find the end. we need the fours to adjust for those first bytes
        let end_of_yaml = contents[4..].find("---").unwrap() + 4;
        let yaml = &contents[..end_of_yaml];
        let YamlHeader {
            title,
            tags,
            layout,
        } = serde_yaml::from_str(yaml)?;
        // next, the contents. we add + to get rid of the final "---\n\n"
        let options = ComrakOptions {
            render: ComrakRenderOptions {
                unsafe_: true, // Allow rendering of raw HTML
                ..ComrakRenderOptions::default()
            },
            extension: ComrakExtensionOptions {
                header_ids: Some(String::new()),
                footnotes: true,
                table: true,
                ..ComrakExtensionOptions::default()
            },
            ..ComrakOptions::default()
        };

        // Content starts after "---\n" (we don't assume an extra newline)
        let contents = comrak::markdown_to_html(&contents[end_of_yaml + 4..], &options);

        // finally, the url.
        let mut url = PathBuf::from(&*filename);
        url.set_extension("html");

        // this is fine
        let url = format!(
            "{:04}/{:02}/{:02}/{}",
            year,
            month,
            day,
            url.to_str().unwrap()
        );

        let published = build_post_time(year, month, day, 0);
        let updated = published.clone();

        // validate for now that the layout is specified as "post"
        match &*layout {
            "post" => (),
            _ => panic!(
                "blog post at path `{}` should have layout `post`",
                path.display()
            ),
        };

        Ok(Self {
            filename,
            title,
            tags,
            year,
            show_year: false,
            month,
            day,
            contents,
            url,
            published,
            updated,
            layout,
        })
    }

    pub fn set_updated(&mut self, seconds: u32) {
        self.updated = build_post_time(self.year, self.month, self.day, seconds);
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AboutMePost {
    pub(crate) filename: String,
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) contents: String,
    pub(crate) url: String,
}

impl AboutMePost {
    pub(crate) fn open(path: &Path) -> eyre::Result<Self> {
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();

        let contents = std::fs::read_to_string(path)?;

        if contents.len() < 5 {
            return Err(eyre!(
                "{path:?} is empty, or too short to have valid front matter"
            ));
        }

        let end_of_yaml = contents[4..].find("---").unwrap() + 4;
        let yaml = &contents[..end_of_yaml];
        let YamlHeader {
            title,
            tags: _,
            layout,
        } = serde_yaml::from_str(yaml)?;
        let options = ComrakOptions {
            render: ComrakRenderOptions {
                unsafe_: true, // Allow rendering of raw HTML
                ..ComrakRenderOptions::default()
            },
            extension: ComrakExtensionOptions {
                header_ids: Some(String::new()),
                footnotes: true,
                table: true,
                ..ComrakExtensionOptions::default()
            },
            ..ComrakOptions::default()
        };

        let contents = comrak::markdown_to_html(&contents[end_of_yaml + 4..], &options);

        match &*layout {
            "aboutme" => (),
            _ => panic!(
                "blog aboutme at path `{}` should have layout `aboutme`",
                path.display()
            ),
        };

        let mut url = PathBuf::from(&filename);
        url.set_extension("html");
        let url = url.to_str().unwrap().to_string();

        Ok(Self {
            filename,
            title,
            contents,
            url,
            layout,
        })
    }
}

fn build_post_time(year: i32, month: u32, day: u32, seconds: u32) -> String {
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDate::from_ymd(year, month, day).and_hms(0, 0, seconds),
        chrono::Utc,
    )
    .to_rfc3339()
}
