
use std::error::Error;
use rss::Channel;
use std::collections::HashMap;

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

#[tokio::main]
async fn main() {
    let mut map: HashMap<i32, BlogPost> = HashMap::new();

    let channel_feed = example_feed().await.unwrap();
    let channel_items = channel_feed.items();
    let mut counter = 1;
    for x in channel_items {
        let title = x.title().unwrap().replace("\u{200a}", " ");
        let pd = x.pub_date().unwrap();
        let pub_date_section = &pd[0..16];
        map.insert(counter, BlogPost {title: title, pub_date: pub_date_section.to_string()});
        counter = counter + 1;
    }
    for (id, blog_post) in map.iter() {
        println!("{} {} {}", id, blog_post.title, blog_post.pub_date)
    }

}