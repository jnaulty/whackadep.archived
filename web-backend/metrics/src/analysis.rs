//! This module implements the code to analyze a repository's dependencies.

use anyhow::Result;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

use crate::db::Db;
use crate::git::Repo;
use crate::rust::RustAnalysis;

#[derive(Serialize, Deserialize)]
/// An analysis result. It contains the commit that was analyzed, as well as the results of the analysis on dependencies.
/// At the moment it only contains analysis results for Rust dependencies.
pub struct Analysis {
    /// The SHA-1 hash indicating the exact commit used to analyze the given repository.
    commit: String,
    /// The result of the rust dependencies analysis
    rust_dependencies: RustAnalysis,
}

/// The analyze function does the following:
/// 1. It initializes a given repository (if not already done previously).
/// 2. It pulls the latest changes.
/// 3. It records the commit pointed by the HEAD of the repository.
/// 4. It runs language-dependent analysis to "extract" information about our dependencies (this step only works for Rust dependencies stuff at the moment).
/// 5. It stores the results in the database.
pub async fn analyze(repo_url: &str, repo_path: &Path) -> Result<()> {
    // 1. initialize repo if not done
    info!("getting diem/diem repo");
    let repo = match Repo::new(repo_path) {
        Ok(repo) => repo,
        Err(_) => {
            info!("cloning {} into {}", repo_url, repo_path.to_string_lossy());
            Repo::clone(repo_url, repo_path).await?
        }
    };

    // 2. pull latest changes on the repo
    info!("pulling latest changes");
    repo.update().await?;

    // 3. get metadata
    let commit = repo.head().await.expect("couldn't get HEAD hash");
    info!("current commit: {}", commit);

    // 4. run analysis for different languages
    // (at the moment we only have Rust)
    let rust_analysis = RustAnalysis::get_dependencies(&repo.repo_folder).await?;

    // 5. store analysis in db
    info!("analysis done, storing in db...");
    let analysis = Analysis {
        commit: commit,
        rust_dependencies: rust_analysis,
    };
    let analysis = bson::to_bson(&analysis).unwrap();
    let document = analysis.as_document().unwrap();
    Db::write(document.to_owned())
}
