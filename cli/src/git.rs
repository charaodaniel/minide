use git2::{Repository, Status, Statuses};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum GitStatus {
    New,
    Modified,
    Deleted,
    Renamed,
    Typechange,
    Ignored,
    Untracked,
    Conflicted,
    Unmodified,
}




pub fn get_file_status(repo: &Repository, path: &Path) -> Option<GitStatus> {
    let statuses = repo.statuses(None).ok()?;
    let relative_path = path.strip_prefix(repo.workdir()?).ok()?;

    if let Some(entry) = statuses.iter().find(|e| Path::new(e.path().unwrap_or_default()) == relative_path) {
        let status = entry.status();
        if status.is_wt_new() {
            return Some(GitStatus::New);
        } else if status.is_wt_modified() {
            return Some(GitStatus::Modified);
        } else if status.is_wt_deleted() {
            return Some(GitStatus::Deleted);
        } else if status.is_wt_renamed() {
            return Some(GitStatus::Renamed);
        } else if status.is_wt_typechange() {
            return Some(GitStatus::Typechange);
        } else if status.is_ignored() {
            return Some(GitStatus::Ignored);
        }
    }

    None
}
