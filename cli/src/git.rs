use git2::Repository;
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

    if let Some(entry) = statuses
        .iter()
        .find(|e| Path::new(e.path().unwrap_or_default()) == relative_path)
    {
        let status = entry.status();
        return Some(if status.is_ignored() {
            GitStatus::Ignored
        } else if status.is_conflicted() {
            GitStatus::Conflicted
        } else if status.is_wt_new() {
            GitStatus::Untracked
        } else if status.is_index_new() {
            GitStatus::New
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            GitStatus::Deleted
        } else if status.is_wt_renamed() || status.is_index_renamed() {
            GitStatus::Renamed
        } else if status.is_wt_typechange() || status.is_index_typechange() {
            GitStatus::Typechange
        } else if status.is_wt_modified() || status.is_index_modified() {
            GitStatus::Modified
        } else {
            GitStatus::Unmodified
        });
    }

    Some(GitStatus::Unmodified)
}
