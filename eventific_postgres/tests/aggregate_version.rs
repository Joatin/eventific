mod util;

use util::setup;
use util::Payload;

#[tokio::test]
async fn it_should_return_current_aggregate_version() -> anyhow::Result<()> {
    let (id, storage) = setup("it_should_return_current_aggregate_version").await?;

    storage
        .save_events(
            &id,
            vec![
                (0, Payload::Created),
                (1, Payload::Created),
                (2, Payload::Created),
            ],
        )
        .await;

    let version = storage.aggregate_version(&id).await?;

    assert_eq!(version, 2);

    Ok(())
}
