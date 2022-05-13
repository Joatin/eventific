use eventific::Storage;
use eventific::Uuid;
use eventific_postgres::PostgresStorage;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Payload {
    Created,
}

pub async fn setup(test_name: &str) -> anyhow::Result<(Uuid, Box<dyn Storage<Payload>>)> {
    let table_name = format!("{}_{}", test_name, Uuid::new_v4().as_u128());
    let aggregate_id = Uuid::new_v4();

    let postgres_storage = PostgresStorage::from_connection_string(
        "host=localhost user=admin password=password",
        &table_name,
    )
    .await?;

    Ok((aggregate_id, Box::new(postgres_storage)))
}

#[allow(dead_code)]
pub async fn setup_with_random_events(
    test_name: &str,
    num_events: usize,
) -> anyhow::Result<(Uuid, Box<dyn Storage<Payload>>)> {
    let (id, storage) = setup(test_name).await?;

    for event_id in 0..num_events {
        storage
            .save_events(&id, vec![(event_id as u64, Payload::Created)])
            .await
            .into_result()?;
    }

    Ok((id, storage))
}
