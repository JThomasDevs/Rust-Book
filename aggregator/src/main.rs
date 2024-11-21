use aggregator::{news_summary, tweet_summary, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    /* This code prints '1 new tweet: horse_ebooks: of course, as you probably
     * already know, people'. */

    news_summary();
    /* This code print 'New article available! (Read more...)'. */

    tweet_summary();
    /* This code prints '1 new tweet: (Read more from @horse_ebooks...)'. */
}
