use rush::greet;

#[tokio::test]
async fn integration_test_example() {
    assert_eq!(greet(), "Hello from Rush Core!");
}
