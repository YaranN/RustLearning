use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, BelongingToDsl};
use ::shoe_store::models::{Product, ProductVariant, Variant};

fn show_product(id: i32, conn: &SqliteConnection) -> Result<(Product, Vec<(ProductVariant, Variant)>), Error> {
    use ::shoe_store::schema::products::dsl::products;
    use ::shoe_store::schema::variants::dsl::variants;

    let product_result =
        products
            .find(id)
            .get_result::<Product>(conn)?;

    let variants_result =
        ProductVariant::belonging_to(&product_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?;

    Ok((product_result, variants_result))
}
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;
use diesel::{TextExpressionMethods, BelongingToDsl, GroupedBy};
use diesel::result::Error;
use ::shoe_store::models::{Product, ProductVariant, Variant};

fn search_products(search: String, conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>, Error> {
    use ::shoe_store::schema::products::dsl::*;
    use ::shoe_store::schema::variants::dsl::variants;

    let pattern = format!("%{}%", search);
    let products_result = 
        products
        .filter(name.like(pattern))
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}