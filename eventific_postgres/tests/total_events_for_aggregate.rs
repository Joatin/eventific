
mod util;

use util::setup;
use util::Payload;

#[tokio::test]
async fn it_should_return_total_events_for_aggregate() -> anyhow::Result<()> {
    let (id, storage) = setup("it_should_return_total_events_for_aggregate").await?;

    storage.save_events(&id, vec![
        (0, Payload::Created),
        (1, Payload::Created),
        (2, Payload::Created),
    ]).await;

    let count = storage.total_events_for_aggregate(&id).await?;

    assert_eq!(count, 3);

    Ok(())
}
