use crate::models::{FeedResult, Post, Request, Serializable_Post, Uri};
use std::error::Error;

/// A feed handler is responsible for
/// - Storing and managing firehose input.
/// - Serving responses to feed requests with `serve_feed`
pub trait FeedHandler {
    fn insert_post(&mut self, post: Post) -> impl std::future::Future<Output = ()> + Send;
    fn delete_post(&mut self, uri: Uri) -> impl std::future::Future<Output = ()> + Send;
    fn like_post(
        &mut self,
        like_uri: Uri,
        liked_post_uri: Uri,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn delete_like(&mut self, like_uri: Uri) -> impl std::future::Future<Output = ()> + Send;
    fn serve_feed(&self, request: Request) -> impl std::future::Future<Output = FeedResult> + Send;
    fn get_all_posts(&self) -> impl std::future::Future<Output = Vec<Serializable_Post>> + Send;
}
