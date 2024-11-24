mod echo;
mod error;
mod health;
mod hello;
mod welcome;

pub use echo::echo;
pub use error::method_not_allowed;
pub use health::health_check;
pub use hello::hello;
pub use welcome::welcome_page;

#[cfg(test)]
mod tests;
