// Controller 模块导出
pub mod auth_controller;
pub mod user_controller;
pub mod article_controller;
pub mod health_controller;

pub use auth_controller::*;
pub use user_controller::*;
pub use article_controller::*;
pub use health_controller::*;

