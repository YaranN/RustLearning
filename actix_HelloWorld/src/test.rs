#[cfg(test)]
mod tests {
    use super::*;
    use actix_Web::{test, web, app};
    use actix_Web::web::Bytes;
    
    #[actix_rt::test]
    async fn test_index_get(){
        let mut app = test::init_service(
            App:new()
            .route("/{name}", web::get().to(greet)),
        )
        .await;
        let req = test::TestRequest::get().uri("/mundo").to_request();
        let resp = test::read_response(&mut app, req).await;
        asser_eq!(resp, Bytes::from_static(b"welcom"));
    }
}