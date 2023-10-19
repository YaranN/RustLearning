use diesel::{ Insertable, Identifiable, Queryable};
use shoe_store::schema::*;
use serde::{Serialize, Deserialize};
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
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "variants"]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name="variants"]
pub struct NewVariant {
    pub name: String,
}

use shoe_store::schema::products_variants;

#[derive(Insertable, Debug)]
#[table_name="products_variants"]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>
}

pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>
}
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}
#[derive(Insertable, Queryable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name="variants"]
pub struct FormVariant {
    pub id: Option<i32>,
    pub name: String
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name="products_variants"]
pub struct FormProductVariant {
    pub id: Option<i32>,
    pub variant_id: Option<i32>,
    pub product_id: i32,
    pub value: Option<String>
}

pub struct FormProductVariantComplete {
    pub variant: Option<FormVariant>,
    pub product_variant: FormProductVariant,
}

pub struct FormProduct {
    pub product: NewProduct,
    pub variants: Vec<FormProductVariantComplete>
}