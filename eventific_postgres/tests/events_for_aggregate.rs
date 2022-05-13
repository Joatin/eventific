
mod util;

use util::setup_with_random_events;
use futures::TryStreamExt;

#[tokio::test]
async fn it_should_return_all_events_for_the_aggregate() -> anyhow::Result<()> {
    let (id, storage) = setup_with_random_events("it_should_return_all_events_for_the_aggregate", 1_000).await?;

    let stream = storage.events_for_aggregate(&id).await;

    let total = stream.try_fold(0, |mut acc, _| async move {
        acc += 1;
        Ok(acc)
    }).await?;

    assert_eq!(total, 1_000);

    Ok(())
}
