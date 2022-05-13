
mod util;

use uuid::Uuid;
use util::setup;
use util::Payload;
use futures::TryStreamExt;

#[tokio::test]
async fn it_should_return_all_aggregate_ids() -> anyhow::Result<()> {
    let (id, storage) = setup("it_should_return_all_aggregate_ids").await?;

    storage.save_events(&id, vec![
        (0, Payload::Created)
    ]).await.into_result()?;

    storage.save_events(&Uuid::new_v4(), vec![
        (0, Payload::Created)
    ]).await.into_result()?;

    storage.save_events(&Uuid::new_v4(), vec![
        (0, Payload::Created)
    ]).await.into_result()?;

    let stream = storage.all_aggregate_ids().await;

    let total = stream.try_fold(0, |mut acc, _| async move {
        acc += 1;
        Ok(acc)
    }).await?;

    assert_eq!(total, 3);

    Ok(())
}
