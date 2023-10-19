use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use ::shoe_store::establish_connection;
use ::shoe_store::models::*;

fn list_products() -> Vec<Product> {
    use ::shoe_store::schema::products::dsl::*;
    let connection = establish_connection();
    products
        .limit(10)
        .load::<Product>(&connection)
        .expect("Error loading products")
}

fn main() {
    println!("The products are: {:#?}", list_products());
}