use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello_v2/Christoff").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "christoff",
            "password": "admin",
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_tickets_no_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AA",
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;

    // hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_tickets_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "christoff",
            "password": "admin",
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AA",
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;

    // hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
