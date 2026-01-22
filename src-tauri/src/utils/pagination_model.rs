use serde::{Deserialize, Serialize};

/// Sorting direction for query results
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

/// Sorting options for fiat ramp queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SortOptions {
    pub column: Option<String>,
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
    pub query: Option<String>,
    pub sort: Option<SortOptions>,
}
