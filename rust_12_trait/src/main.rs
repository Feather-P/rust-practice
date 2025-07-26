use crate::aggregator::{SocialPost, Summary};

mod aggregator;



fn main() {
    let post = SocialPost{
        content: String::from("Fox Jumped though thr fire."),
        username: String::from("Hitsubi"),
        reply: false,
        repost: false
    };

    let post_summarize = post.summarize();
    println!("{post_summarize}")
}
