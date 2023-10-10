use mongodb::Database;
use crate::models::item_model::{Item, ItemStatus};

pub async fn seed_db(db: &Database) {
    // Runs on first app start-up
    let coll = db.collection::<Item>("items");
    let doc_count = coll.count_documents(None, None).await.unwrap();

    if doc_count == 0 {
        println!("Seeding DB with initial items...");

        let seeding_doc = Item {
            id: None,
            name: "Example Item 1".to_string(),
            image: std::env::var("SEED_IMAGE_URL").ok(),
            quantity: 0,
            borrower_ids: vec![],
            status: ItemStatus::Available
        };

        let _ = coll.insert_one(seeding_doc, None).await.unwrap();
        println!("Successfully Seeded DB with initial items.");
    } else {
        println!("DB seeding not required.");
    }
}
