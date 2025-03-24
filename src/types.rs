#[derive(serde::Serialize, Clone)]
pub struct Link {
    pub id: String,
    pub timestamp: u64,
    pub url: String,
}
