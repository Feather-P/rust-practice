pub struct Post {
    status: Option<Box<dyn Status>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            status: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn content(&self) -> &str {
        self.status.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.status.take() {
            self.status = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.status.take() {
            self.status = Some(s.approve());
        }
    }

    pub fn make_draft(&mut self) {
        if let Some(s) = self.status.take() {
            self.status = Some(s.draft())
        }
    }
}

struct Draft {}

impl Status for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn Status> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn Status> {
        self
    }
    fn draft(self: Box<Self>) -> Box<dyn Status> {
        self
    }
}

struct PendingReview {}

impl Status for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn Status> {
        Box::new(Published {})
    }
    fn draft(self: Box<Self>) -> Box<dyn Status> {
        Box::new(Draft {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn Status> {
        self
    }
}

struct Published {}

impl Status for Published {
    fn approve(self: Box<Self>) -> Box<dyn Status> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn Status> {
        self
    }
    fn draft(self: Box<Self>) -> Box<dyn Status> {
        Box::new(Draft {})
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

trait Status {
    fn draft(self: Box<Self>) -> Box<dyn Status>;
    fn request_review(self: Box<Self>) -> Box<dyn Status>;
    fn approve(self: Box<Self>) -> Box<dyn Status>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}