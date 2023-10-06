
/* ------------------------------- 주문 취소 함수 ------------------------------ */
//세션을 통해서 지금까지 해당 고객이 주문한 모든 주문 불러오기
pub async fn cancel_order(
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    
    //find session's customer_id
    let customer_id = session.get::<uuid::Uuid>("customer_id").unwrap();
    println!("customer_id: {}", customer_id);

    //find all order_id with customer_id
    let order_id = order::Entity::find()
        .filter(order::Column::CustomerId.contains(customer_id.to_string()))
        .all(&conn)
        .await
        .unwrap()
        .into_iter()
        .map(|item| item.order_id)
        .collect::<Vec<i32>>();

    //show the customer's order list
    for id in order_id {
        let order_detail_id = order_detail::Entity::find()
            .filter(order_detail::Column::OrderId.contains(id.to_string()))
            .all(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|item| item.order_detail_id)
            .collect::<Vec<i32>>();

        for detail_id in order_detail_id {
            let order_detail_model = order_detail::ActiveModel {
                order_detail_id: ActiveValue::Set(detail_id),
                completed: ActiveValue::Set(0),
                ..Default::default()
            };

            order_detail_model.update(&conn).await.unwrap();
        }
    }
    session.remove("customer_id");
    println!("세션이 만료되었습니다.");
}
/* -------------------------------------------------------------------------- */