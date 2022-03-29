
use std::error::Error;
use rss::Channel;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct BlogPost {
    title: String,
    pub_date: String,
}

async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://sebhulse.medium.com/feed")
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
        map.insert(counter, BlogPost {title: title, pub_date: pub_date_section.to_string()});
        counter = counter + 1;
    }
    Ok(map)
}

fn read_index_input_html() -> Result<String, Box<dyn Error>> {
    let path = Path::new("../../index_copy.html");

    let mut file = File::open(&path).expect("Couldn't open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read file to string");
    Ok(s)
}

fn insert_blog_posts(input_html: String) -> Result<String, Box<dyn Error>> {
    let result: String = input_html[..].replace("<!-- insert posts here -->", "wow!!");
    Ok(result)
}

#[tokio::main]
async fn main() {
    let channel_feed = example_feed().await.unwrap();
    
    let blog_posts = parse_rss_feed(channel_feed).unwrap();

    for (id, blog_post) in blog_posts.iter() {
        println!("{} {} {}", id, blog_post.title, blog_post.pub_date)
    }

    let html = read_index_input_html().unwrap();
    println!("{:?}", html);
    let html_with_blog_posts = insert_blog_posts(html).unwrap();
    println!("{:?}", html_with_blog_posts);

}