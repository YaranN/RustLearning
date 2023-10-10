use diesel::Insertable;
use shoe_store::schema::*;
use ::shoe_store::models::NewProduct;
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::RunQueryDsl;
#[derive(Insertable, Debug)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}
fn create_product(new_product: NewProduct, conn: &PgConnection) -> Result<usize, Error>  {
    use ::shoe_store::schema::products::dsl::*;
    diesel::insert_into(products)
        .values(new_product)
        .execute(conn)
}