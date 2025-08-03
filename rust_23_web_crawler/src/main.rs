use trpl::{race, Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url1 = &args[1];
    let url2 =&args[2];

    trpl::run(async {
        let (url, maybe_title) = match race(page_title(&url1), page_title(&url2)).await {
            Either::Left(title1) => title1,
            Either::Right(title2) => title2
        };
        println!("{} returned first!", url);
        match maybe_title {
            Some(title) => println!("{} with title: {}",url, title),
            None => println!("This site has no title")
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
        let response = trpl::get(url).await;
        let response_text = response.text().await;
        let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
    }