mod echo;
mod error;
mod health;
mod hello;
mod welcome;

pub use echo::echo;
pub use error::{method_not_allowed, not_found};
pub use health::health_check;

pub use hello::hello;
pub use welcome::welcome_page;

use actix_web::{web, HttpResponse, Responder};
use crate::models::Message;

#[cfg(test)]
mod tests;
