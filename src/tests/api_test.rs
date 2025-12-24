use reqwest::blocking;

#[test]
fn api_call() {
    let response = blocking::get("https://httpbin.org/get").unwrap();
    let body = response.text().unwrap();
    println!("Response Body: {}", body);
}
