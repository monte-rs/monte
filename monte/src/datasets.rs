//! Datasets
//! Use all scipy datasets https://docs.scipy.org/doc/scipy/reference/datasets.html
//! Include downloads for other ones (MUST be available long-term from a trustworthy source).
//! Use tokio to download datasets. Maybe chop up dataset into chunks and request each separately?

pub mod download;
pub mod generate;
