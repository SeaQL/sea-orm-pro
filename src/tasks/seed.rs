//! This task implements data seeding functionality for initializing new
//! development/demo environments.
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task seed_data refresh:true
//! ```

use encoding_rs::{UTF_16LE, WINDOWS_1252};
use loco_rs::{db, prelude::*};
use migration::Migrator;
use sea_orm::DbConn;
use serde::de::DeserializeOwned;

use crate::models;

#[allow(clippy::module_name_repetitions)]
pub struct SeedData;
#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_data".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let refresh = vars
            .cli_arg("refresh")
            .is_ok_and(|refresh| refresh == "true");
        let db = &app_context.db;

        // Run migration before seeding database
        if refresh {
            db::reset::<Migrator>(db).await?;
        } else {
            db::migrate::<Migrator>(db).await?;
        }

        // Start: Scaffold //
        // Some boilerplate
        const CRLF: csv::Terminator = csv::Terminator::CRLF;
        const DOLLAR_SIGN: csv::Terminator = csv::Terminator::Any(b'$');
        macro_rules! seed_table_win_1252 {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, WINDOWS_1252, b'\t', CRLF, db)
                    .await?;
            };
        }
        macro_rules! seed_table_utf_16 {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, UTF_16LE, b'\t', CRLF, db).await?;
            };
        }
        macro_rules! seed_table_utf_16_tilde {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, UTF_16LE, b'~', DOLLAR_SIGN, db)
                    .await?;
            };
        }

        // Seed each table in sequence
        seed_table_win_1252!(customer, "Customer.tsv");
        seed_table_win_1252!(address, "Address.tsv");
        seed_table_win_1252!(customer_address, "CustomerAddress.tsv");
        seed_table_win_1252!(sales_order_header, "SalesOrderHeader.tsv");
        seed_table_win_1252!(product_category, "ProductCategory.tsv");
        seed_table_utf_16_tilde!(product_model, "ProductModel.tsv");
        seed_table_win_1252!(product, "Product.tsv");
        seed_table_win_1252!(sales_order_detail, "SalesOrderDetail.tsv");
        seed_table_utf_16!(product_description, "ProductDescription.tsv");
        seed_table_win_1252!(
            product_model_product_description,
            "ProductModelProductDescription.tsv"
        );

        println!("All Tables Completed Seeding!");
        // End: Scaffold //

        Ok(())
    }
}

// Start: Scaffold //
async fn seed_table<T, A>(
    csv: &str,
    encoding: &'static encoding_rs::Encoding,
    delimiter: u8,
    terminator: csv::Terminator,
    db: &DbConn,
) -> Result<()>
where
    T: DeserializeOwned + ModelTrait + IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior,
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    println!("Reading: {csv:?}");
    // Read data from CSV
    let file = std::fs::OpenOptions::new().read(true).open(format!(
        "{}/migration/AdventureWorks-2012-LT-Script/{}",
        env!("CARGO_MANIFEST_DIR"),
        csv
    ))?;
    let transcoded = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .terminator(terminator)
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(transcoded);
    // Collect into SeaORM's active model
    let mut active_models: Vec<A> = Vec::new();
    for result in rdr.deserialize() {
        let model: T = result.unwrap();
        active_models.push(model.into_active_model());
    }
    // Insert active model in chunks
    for models in active_models.chunks(100) {
        let models = models.iter().cloned();
        <A::Entity as EntityTrait>::insert_many(models)
            .exec(db)
            .await?;
    }
    println!("Seeding Completed: {csv:?}");
    Ok(())
}
// End: Scaffold //
