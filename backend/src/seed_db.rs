use std::env;
use bcrypt::DEFAULT_COST;
use mongodb::Database;
use branca::Branca;
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::models::user_model::{UserRawPhysicalToken, User, UserRoleType};
use crate::models::location_model::{Location, LocationStatus, LocationType};
use crate::models::item_model::{Item, ItemStatus};

pub async fn seed_db(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    // Runs on first app start-up.
    // Creates (order specific): first (root) admin user; first location; first item.

    let users_coll = db.collection::<User>("users");
    let users_count = users_coll.count_documents(None, None).await?;

    let locations_coll = db.collection::<Location>("locations");
    let locations_count = locations_coll.count_documents(None, None).await?;

    let items_coll = db.collection::<Item>("items");
    let items_count = items_coll.count_documents(None, None).await?;

    let initial_username = env::var("DEFAULT_USERNAME").expect("No DEFAULT_USERNAME provided in the env variables.");
    let initial_password = env::var("DEFAULT_PASSWORD").expect("No DEFAULT_PASSWORD provided in the env variables.");
    let pass_hash = bcrypt::hash(initial_password, DEFAULT_COST).unwrap();

    if users_count == 0 && locations_count == 0 && items_count == 0 {
        println!("Seeding DB with initial user...");

        // ID Token Creation
        let token_obj = UserRawPhysicalToken {
            user_id: ObjectId::new(),
            username: initial_username.clone(),
            pass_hash: pass_hash.clone()
        };
        let token_raw = serde_json::to_string(&token_obj)?;

        let physical_id_qr_token: String = Branca::new(
            env::var("BRANCA_KEY").expect("No BRANCA_KEY provided. See the example ENV file for how to set the variable.").as_bytes()
        )
            .unwrap()
            .encode(&token_raw.into_bytes())
            .unwrap();

        users_coll.insert_one(User {
            id: Some(token_obj.user_id.clone()),
            username: initial_username.clone(),
            pass_hash,
            full_name: "Joe Bloggs".to_string(),
            role: UserRoleType::Admin,
            physical_id_qr_token,
        }, None).await.unwrap();

        println!("Seeding DB with initial location...");
        let location_insert_result = locations_coll.insert_one(Location {
            id: None,
            location_type: LocationType::Room,
            location_identifier: "RM14".to_string(),
            rbac_min_level: UserRoleType::User,
            max_capacity: None,
            location_status: LocationStatus::Available,
        }, None)
        .await
        .unwrap();

        println!("Seeding DB with initial items...");
        let _ = items_coll.insert_one(Item {
            id: None,
            name: "Example Item 1".to_string(),
            image_uri: env::var("SEED_IMAGE_URL").ok(),
            quantity: 0,
            borrower_ids: vec![
                token_obj.user_id.clone()
            ],
            location_id: location_insert_result.inserted_id.as_object_id().unwrap(),
            status: ItemStatus::Available,
        }, None).await;

        println!("[!] Seeded DB with all initial data!");

    }

    Ok(())
}
