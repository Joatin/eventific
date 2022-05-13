
#[derive(Debug)]
pub enum SaveEventsResult {
    Ok,
    VersionConflict,
    Error(anyhow::Error)
}

impl SaveEventsResult {

    pub fn is_ok(&self) -> bool {
        match self {
            SaveEventsResult::Ok => true,
            SaveEventsResult::VersionConflict => false,
            SaveEventsResult::Error(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn is_version_conflict(&self) -> bool {
        match self {
            SaveEventsResult::Ok => false,
            SaveEventsResult::VersionConflict => true,
            SaveEventsResult::Error(_) => false,
        }
    }

    pub fn into_result(self) -> anyhow::Result<()> {
        self.into()
    }
}

impl From<SaveEventsResult> for anyhow::Result<()> {
    fn from(event_result: SaveEventsResult) -> Self {
        match event_result {
            SaveEventsResult::Ok => Ok(()),
            SaveEventsResult::VersionConflict => Err(anyhow::Error::msg("Version Conflict")),
            SaveEventsResult::Error(err) => Err(err),
        }
    }
}
