use maga::RandomTweetGenerator;
use dioxus::{prelude::*};
fn main() {
    let generator = RandomTweetGenerator::default();
    dioxus::desktop::launch_with_props(app, generator, |c| {
        c.with_window(|wb| wb.with_title("🇺🇸 MAGA! 🇺🇸"))
    });
}

fn app<'a>(cx: Scope<'a,RandomTweetGenerator<'_>>) -> Element<'a> {
    let  (tweet, set_tweet) = use_state(&cx, || String::new());
    cx.render(rsx! (
        head {
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" }
        }
        body {
            div { 
                class:"container mt-5",
                div {
                    class: "d-grid gap-2",
                    button {
                        class: "btn btn-outline-primary btn-block",
                        onclick: move |_| {
                            let generator = cx.props;
                            let tweet = String::from(generator.get_random_tweet().text());
                            set_tweet(tweet);
                        },
                        "🇺🇸 MAGA! 🇺🇸",
                    }
                },
                hr{},
                (!tweet.is_empty()).then(|| {
                    rsx!{
                        blockquote {
                            class: "blockquote",
                            p {
                                "{tweet}"
                            }
                        }
                    }
                })
            }

        }
    ))
}