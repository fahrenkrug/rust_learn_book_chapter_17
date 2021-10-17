use oop::Post;

pub fn run(){
    post_example();
}

fn post_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
    println!("I ate a salad for lunch today.");
}

fn rustified_post_example() {
    let mut post = rustified_state::Post::new();
    post.add_text("I ate a salad for lunch today.");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}

mod rustified_state {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content
            }
        }
    }
}