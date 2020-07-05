
use serde::{Serialize};

/**
 * The Pokemon domain model
 */
#[derive(Debug, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}