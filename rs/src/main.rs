use chrono::offset::Utc;
use minify_html::{minify, Cfg};
use rss::Channel;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// TODO: input -i output -o command line argmuents
// TODO: get size of webpage (css and html)
// TODO: Devops
// - minify css and compile main.rs on push
// - schedule main.rs to run every day
// TODO: Improve styling of website - more focus on landing page, with links to other pages in corners (or hamburger menu on mobile)

struct BlogPost {
    title: String,
    pub_date: String,
    link: String,
}

async fn get_channel_feed(input_feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(input_feed).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn parse_rss_feed(channel_feed: Channel) -> Result<HashMap<i32, BlogPost>, Box<dyn Error>> {
    let mut map: HashMap<i32, BlogPost> = HashMap::new();
    let channel_items = channel_feed.items();
    let mut counter = 1;

    for x in channel_items {
        let title = x.title().unwrap().replace("\u{200a}", " ");
        let pd = x.pub_date().unwrap();
        let pub_date_section = &pd[0..16];
        let link = x.link().unwrap();
        map.insert(
            counter,
            BlogPost {
                title: title,
                pub_date: pub_date_section.to_string(),
                link: link.to_string(),
            },
        );
        counter = counter + 1;
    }
    Ok(map)
}

fn read_html_file(html_file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(html_file_path);
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut result = String::new();

    file.read_to_string(&mut result)
        .expect("Couldn't read file to string");
    Ok(result)
}

fn insert_blog_posts(html_file: String, posts: String) -> Result<String, Box<dyn Error>> {
    let result: String = html_file[..].replace("<!-- insert posts here -->", &posts[..]);
    Ok(result)
}

fn create_blog_posts(blog_posts: HashMap<i32, BlogPost>) -> Result<String, Box<dyn Error>> {
    let mut post = String::new();
    let timestamp = Utc::now().to_rfc2822();
    let timestamp_section = &timestamp[0..16];
    post.push_str(&format!(
        "<p class='text-lg text-slate-200 pt-4'>Here are my 6 most recent posts (of {}):</p>\n",
        blog_posts.len()
    ));
    post.push_str(&format!(
        "<p class='text-slate-400'>Updated {}</p>\n",
        timestamp_section
    ));

    for index in 1..7 {
        let title = &blog_posts.get(&index).unwrap().title;
        let link = &blog_posts.get(&index).unwrap().link;
        let pub_date = &blog_posts.get(&index).unwrap().pub_date;
        post.push_str(&format!("<a target='_blank' href='{}'><div class='blog-card'>{}<div class='text-slate-400 text-md'>{}</div></div></a>\n", &link, &title, &pub_date));
    }
    Ok(post)
}

fn minify_html(html: String) -> Result<String, Box<dyn Error>> {
    let minified_html = minify(html.as_bytes(), &Cfg::new());
    let result = std::str::from_utf8(&minified_html).unwrap().to_string();
    Ok(result)
}

fn write_html_file(html: String, html_file_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(html_file_path);
    let mut file = File::create(path).expect("Couldn't create file");
    file.write_all(html.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let input_html_file_path = std::env::args().nth(1).expect("No input path given");
    let output_html_file_path = std::env::args().nth(2).expect("No output path given");

    if input_html_file_path.contains("projects") && input_html_file_path.contains("html") {
        let input_feed = "https://sebhulse.medium.com/feed";
        let channel_feed = get_channel_feed(input_feed).await.unwrap();
        let blog_posts = parse_rss_feed(channel_feed).unwrap();
        let posts = create_blog_posts(blog_posts).unwrap();
        let html = read_html_file(&input_html_file_path[..]).unwrap();
        let html_with_blog_posts = insert_blog_posts(html, posts).unwrap();
        let minified_html = minify_html(html_with_blog_posts).unwrap();
        write_html_file(minified_html, &output_html_file_path[..]).unwrap();
        println!(
            "{} {}",
            "SUCCESS. HTML minified and blog posts added for ", output_html_file_path
        )
    } else if input_html_file_path.contains("html") {
        let html = read_html_file(&input_html_file_path[..]).unwrap();
        let minified_html = minify_html(html).unwrap();
        write_html_file(minified_html, &output_html_file_path[..]).unwrap();
        println!(
            "{} {}",
            "SUCCESS. HTML minified for ", output_html_file_path
        )
    } else {
        println!(
            "{} {} {}",
            "ERROR. ", input_html_file_path, " is not an HTML file",
        )
    }
}
