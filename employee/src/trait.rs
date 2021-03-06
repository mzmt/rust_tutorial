pub fn notify<T: summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // {}さんからもっと読む
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    // let article = NewsArticle {
    //     // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     // ピッツバーグ、ペンシルベニア州、アメリカ
    //     location: String::from("Pittsburgh, PA, USA"),
    //     // アイスバーグ
    //     author: String::from("Iceburgh"),
    //     // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
    //     content: String::from("The Pittsburgh Penguins once again are the best
    //     hockey team in the NHL."),
    // };

    // // 新しい記事が利用可能です！ {}
    // println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("ryohei"),
        content: String::from("aaaaaaaaa"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
