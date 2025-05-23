use state_pattern::Post;

// The final functionality will look like this:
//     1. A blog post starts as an empty draft.
//     2. When the draft is done, a review of the post is requested.
//     3. When the post is approved, it gets published.
//     4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

// Possible extra features:
//     1. Add a reject method that changes the post’s state from PendingReview back to Draft.
//     2. Require two calls to approve before the state can be changed to Published.
//     3. Allow users to add text content only when a post is in the Draft state.
//        Hint: have the state object responsible for what might change about the content
//        but not responsible for modifying the Post.

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
