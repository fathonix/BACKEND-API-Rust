use actix_web::{HttpResponse, Responder, web};
use diesel::update;
use diesel::dsl::{delete, insert_into};

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::*;
use crate::Pool;
use crate::schema::products;
use crate::schema::products::dsl;

sql_function! {
    fn last_insert_id() -> Unsigned<BigInt>
}

pub async fn get_item(
    db: web::Data<Pool>,
    item_id: web::Path<u64>,
) -> impl Responder {
    let result = dsl::products
        .find(item_id.into_inner())
        .get_result::<Product>(&mut db.get().unwrap());

    match result {
        Ok(item) => {
            let output = OutputProduct {
                status: true,
                message: "Success get detail data",
                data: Some(item),
            };
            HttpResponse::Ok().json(output)
        }
        Err(e) => {
            let error = OutputError {
                status: false,
                message: &*e.to_string(),
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

pub async fn get_items(db: web::Data<Pool>) -> impl Responder {
    let result =
        dsl::products.load::<Product>(&mut db.get().unwrap());

    match result {
        Ok(items) => {
            let output = OutputProducts {
                status: true,
                message: "success get all products",
                data: Some(items),
            };
            HttpResponse::Ok().json(output)
        }
        Err(e) => {
            let error = OutputError {
                status: false,
                message: &*e.to_string(),
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

pub async fn create_item(
    db: web::Data<Pool>,
    input_item: web::Json<InputProduct>,
) -> impl Responder {
    let timestamp = chrono::Local::now().naive_local();
    let new_item = NewProduct {
        name: input_item.name.clone(),
        price: input_item.price,
        created_at: timestamp,
        updated_at: timestamp,
    };
    
    let create = insert_into(dsl::products)
        .values(new_item)
        .execute(&mut db.get().unwrap());

    let result = match create {
        Ok(_) => dsl::products
            .find(last_insert_id())
            .get_result::<Product>(&mut db.get().unwrap()),
        Err(e) => Err(e)
    };

    match result {
        Ok(item) => {
            let output = OutputProduct {
                status: true,
                message: "success create product",
                data: Some(item),
            };
            HttpResponse::Ok().json(output)
        }
        Err(e) => {
            let error = OutputError {
                status: false,
                message: &*e.to_string(),
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

pub async fn update_item(
    db: web::Data<Pool>,
    item_id: web::Path<u64>,
    input_item: web::Json<InputProduct>,
) -> impl Responder {
    let id = item_id.into_inner();
    let query = update(dsl::products)
        .filter(products::id.eq(id))
        .set((
            products::name.eq(input_item.name.clone()),
            products::price.eq(input_item.price),
            products::updated_at.eq(chrono::Local::now().naive_local())
        ))
        .execute(&mut db.get().unwrap());

    let result = match query {
        Ok(_) => dsl::products
            .find(id)
            .get_result::<Product>(&mut db.get().unwrap()),
        Err(e) => Err(e)
    };

    match result {
        Ok(item) => {
            let output = OutputProduct {
                status: true,
                message: "Success update data",
                data: Some(item),
            };
            HttpResponse::Ok().json(output)
        }
        Err(e) => {
            let error = OutputError {
                status: false,
                message: &*e.to_string(),
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

pub async fn delete_item(
    db: web::Data<Pool>,
    item_id: web::Path<u64>,
) -> impl Responder {
    let process = delete(dsl::products)
        .filter(products::id.eq(item_id.into_inner()))
        .execute(&mut db.get().unwrap());

    match process {
        Ok(_) => {
            let output = OutputProduct {
                status: true,
                message: "success delete product",
                data: None,
            };
            HttpResponse::Ok().json(output)
        }
        Err(e) => {
            let error = OutputError {
                status: false,
                message: &*e.to_string(),
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}