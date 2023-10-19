#[macro_use]
extern crate diesel;

use diesel::{PgConnection, Connection, QueryDsl, RunQueryDsl};
use anyhow::Result;
use ::shoe_store::models::FormProduct;
no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

fn update_product(product_id: i32, form_product: FormProduct, conn: &SqliteConnection) -> Result<i32> {
    use ::shoe_store::schema::products::dsl::products;
    use ::shoe_store::schema::variants;
    use ::shoe_store::schema::products_variants::dsl::products_variants;

    // We begin a transaction, just to make sure everything runs successfully
    conn.transaction(|| {
        diesel::update(products.find(product_id))
            .set(&form_product.product)
            .execute(conn)?;

        // We just loop through all variants available
        for mut form_product_variant in form_product.variants {
            // If there's no variant id, we probably need to insert a new one.
            if form_product_variant.product_variant.variant_id.is_none() {
                diesel::insert_into(variants::dsl::variants)
                    .values(form_product_variant.variant)
                    .execute(conn)?;

                let last_variant_id: i32 =
                        diesel::select(last_insert_rowid).first(conn)?;

                form_product_variant.product_variant.variant_id = Some(last_variant_id);            
            }
            // We have an id, so we should update the variant product.
            if let Some(product_variant_id) = form_product_variant.product_variant.id {
                diesel::update(products_variants.find(product_variant_id))
                    .set(&form_product_variant.product_variant)
                    .execute(conn)?;
            }
        }

        Ok(product_id)
    })
}
use diesel::sqlite::SqliteConnection;
use anyhow::Result;
use diesel::{QueryDsl, RunQueryDsl};

fn delete_product(id: i32, conn: &SqliteConnection) ->  Result<i32> {
    use ::shoe_store::schema::products::dsl::products;
    diesel::delete(products.find(id))
        .execute(conn)?;

    Ok(id)
}