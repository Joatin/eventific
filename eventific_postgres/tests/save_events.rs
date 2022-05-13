mod util;

use util::setup;
use util::Payload;

#[tokio::test]
async fn it_should_save_events() -> anyhow::Result<()> {
    let (id, storage) = setup("it_should_save_events").await?;

    let result = storage.save_events(&id, vec![(0, Payload::Created)]).await;

    assert!(result.is_ok(), "actual value was: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn it_should_return_correct_error_for_duplicated_events() -> anyhow::Result<()> {
    let (id, storage) = setup("it_should_return_correct_error_for_duplicated_events").await?;

    // First successfull
    storage.save_events(&id, vec![(0, Payload::Created)]).await;

    // Second time we add event 0 should fail with [SaveEventsResult::VersionConflict]
    let result = storage.save_events(&id, vec![(0, Payload::Created)]).await;

    assert!(
        result.is_version_conflict(),
        "actual value was: {:?}",
        result
    );

    Ok(())
}
