#![allow(dead_code)]

use crate::{Customer, Item, Order};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// async insertion is done in random order, so records has to be sorted before testing
pub fn sort_records_by_ids<T>(records: Vec<T>, ids: Vec<uuid::Uuid>) -> Vec<T> {
    let mut indexed_records = ids
        .iter()
        .zip(records.into_iter())
        .collect::<Vec<(&uuid::Uuid, T)>>();
    indexed_records.sort_unstable_by_key(|(i, _)| *i);
    indexed_records
        .into_iter()
        .map(|(_, r)| r)
        .collect::<Vec<T>>()
}

#[derive(Clone)]
pub struct MockTable<T>
where
    T: Clone,
{
    ids_by_name: Arc<Mutex<HashMap<String, uuid::Uuid>>>,
    records: Arc<Mutex<Vec<T>>>,
}

// tentative mock 'database' that can store records to get tested later on.
// TODO: use database to make it work with async
impl<T> MockTable<T>
where
    T: Clone,
{
    pub fn new(ids_by_name: Vec<(String, uuid::Uuid)>) -> Self {
        let ids_by_name = HashMap::from_iter(ids_by_name.into_iter());

        MockTable {
            ids_by_name: Arc::new(Mutex::new(ids_by_name)),
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_records(&self) -> Vec<T> {
        self.records.lock().unwrap().clone()
    }
}

impl MockTable<Item> {
    // simply registers the record and returns pre-reistered `id` for testing purpose
    pub async fn insert(&mut self, record: Item) -> Result<uuid::Uuid> {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let ids_by_name = self.ids_by_name.lock().unwrap();
        let id = ids_by_name
            .get(&record.name)
            .map(|i| i.to_owned())
            .ok_or_else(|| anyhow::anyhow!("insert failed"));
        let mut records = self.records.lock().unwrap();
        records.push(record);

        id
    }
}

impl MockTable<Customer> {
    // simply registers the record and returns pre-reistered `id` for testing purpose
    pub async fn insert(&mut self, record: Customer) -> Result<uuid::Uuid> {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let ids_by_name = self.ids_by_name.lock().unwrap();
        let id = ids_by_name
            .get(&record.name)
            .map(|i| i.to_owned())
            .ok_or_else(|| anyhow::anyhow!("insert failed"));
        let mut records = self.records.lock().unwrap();
        records.push(record);

        id
    }
}

impl MockTable<Order> {
    // simply registers the record and returns pre-reistered `id` for testing purpose
    pub async fn insert(&mut self, record: Order) -> Result<uuid::Uuid> {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let ids_by_name = self.ids_by_name.lock().unwrap();
        let id = ids_by_name
            .get(&record.id.to_string())
            .map(|i| i.to_owned())
            .ok_or_else(|| anyhow::anyhow!("insert failed"));
        let mut records = self.records.lock().unwrap();
        records.push(record);

        id
    }
}
