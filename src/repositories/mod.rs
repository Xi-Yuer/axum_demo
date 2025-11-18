// Repository 模块导出
pub mod user_repository;
pub mod article_repository;

// 避免 glob re-export 冲突，使用模块路径访问
pub use user_repository as user;
pub use article_repository as article;

