use std::error::Error;
use rss::Channel;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// TODO: input -i output -o command line argmuents 
// TODO: minify_html 
// TODO: output to html file

struct BlogPost {
    title: String,
    pub_date: String,
    link: String,
}

async fn get_channel_feed(input_feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(input_feed)
        .await?
        .bytes()
        .await?;
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
        map.insert(counter, BlogPost {title: title, pub_date: pub_date_section.to_string(), link: link.to_string()});
        counter = counter + 1;
    }
    Ok(map)
}

fn read_input_html_file(input_file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(input_file_path);
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut result = String::new();

    file.read_to_string(&mut result).expect("Couldn't read file to string");
    Ok(result)
}

fn insert_blog_posts(html_file: String, posts: String) -> Result<String, Box<dyn Error>> {
    let result: String = html_file[..].replace("<!-- insert posts here -->", &posts[..]);
    Ok(result)
}

fn create_blog_posts(blog_posts: HashMap<i32, BlogPost>) -> Result<String, Box<dyn Error>> {
    let mut post = String::new();

    for index in 1..5 {
        let title = &blog_posts.get(&index).unwrap().title;
        let link = &blog_posts.get(&index).unwrap().link;
        let pub_date = &blog_posts.get(&index).unwrap().pub_date;
        post.push_str(&format!("<a target='_blank' href={}><div class='blog-card'>{}<div class='text-slate-400 text-md'>{}</div></div></a>\n", &link, &title, &pub_date));
    }
    Ok(post)
}

#[tokio::main]
async fn main() {
    let input_feed = "https://sebhulse.medium.com/feed";
    let channel_feed = get_channel_feed(input_feed).await.unwrap();
    let blog_posts = parse_rss_feed(channel_feed).unwrap();

    let posts = create_blog_posts(blog_posts).unwrap();
    // println!("posts: {}", posts);

    let input_file_path = "../../index_copy.html";

    let html = read_input_html_file(input_file_path).unwrap();
    // println!("{:?}", html);
    let html_with_blog_posts = insert_blog_posts(html, posts).unwrap();
    println!("{:?}", html_with_blog_posts);

}