fn main() {
    //just a plain useless main function
}

#[cfg(test)]
mod tests {
    use rust_28_blog::Post;

    #[test]
    fn all() {
        let mut my_post = Post::new();
        my_post.add("Ciallo~");
        assert_eq!(my_post.content(), "");
        my_post.request_review();
        assert_eq!(my_post.content(), "");
        my_post.approve();
        assert_eq!(my_post.content(), "Ciallo~");
        my_post.make_draft();
        assert_eq!(my_post.content(), "")
    }
}
