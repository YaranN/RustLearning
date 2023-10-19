use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::{RunQueryDsl, GroupedBy, QueryDsl, BelongingToDsl};
use ::shoe_store::models::{Product, ProductVariant, Variant};

fn list_products(conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use ::shoe_store::schema::products::dsl::products;
    use ::shoe_store::schema::variants::dsl::variants;

    let products_result = 
        products
        .limit(10)
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}