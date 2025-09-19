use axum_rest_v0::routers::create_routes;
use axum_rest_v0::state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let app = create_routes(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listen on http://{}/", &addr);
    println!("Using In-Memory HashMap Database ");
    println!("Available endpoints: ");
    println!("\nCheck Health:");
    println!("curl -s -X GET http://{}/", &addr);
    println!("curl -s -X GET http://{}/hello", &addr);
    println!("\nGet all users:");
    println!("curl -s -X GET http://{}/users", &addr);
    println!("\nCreate user:");
    println!("curl -s -X POST http://{}/users", &addr);
    println!("\nGet User by Id:");
    println!("curl -s -X GET http://{}/users/", &addr);
    println!("\nUpdate user by Id:");
    println!("curl -s -X PUT http://{}/users/ ", &addr);
    println!("\nDelete user by id:");
    println!("curl -s -X DELETE http://{}/users/ ", &addr);
    println!("Press Ctlr+C to stop server");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
