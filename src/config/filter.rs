use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Filter {
    pub contains: Option<String>,
}

impl Filter {
    pub fn passes(&self, gen: &str) -> bool {
        self.contains.as_deref().map(|contains| gen.contains(contains)).unwrap_or(true)
    }
}
