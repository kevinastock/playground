use anyhow::Result;
use testing::harness::TestHarness;

#[tokio::test]
async fn single_client_receives_expected_message() -> Result<()> {
    let harness = TestHarness::start().await?;
    let mut client = harness.connect_client().await?;

    let message = client.say_hello("Alice").await?;
    assert_eq!(message, "Hello, Alice!");

    harness.shutdown().await
}

#[tokio::test]
async fn multiple_clients_can_call_server() -> Result<()> {
    let harness = TestHarness::start().await?;

    let mut first_client = harness.connect_client().await?;
    let mut second_client = harness.connect_client().await?;

    let (first, second) = tokio::join!(
        first_client.say_hello("Ada"),
        second_client.say_hello("Grace")
    );

    assert_eq!(first?, "Hello, Ada!");
    assert_eq!(second?, "Hello, Grace!");

    harness.shutdown().await
}
