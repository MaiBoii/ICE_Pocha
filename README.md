# 🧊 ICE_POCHA

The Order Automation System for our University Festival Pub.

## Backend
#### 
- [Axum](https://github.com/tokio-rs/axum)
- [Sea-orm](https://github.com/SeaQL/sea-orm)

## Todo
- [x] Sending and receiving orders
- [x] Calculate the total price and margin by menu_id and quantity
- [x] Discern the customer with session cookie
- [x] Destroy the session cookie when customer payed and left and complete the order_detail
- [x] Separate the date of order_detail when staff Closed the order and save the day's margin
- [ ] Real-time order and confirmation notifications using the WebSocket protocol

