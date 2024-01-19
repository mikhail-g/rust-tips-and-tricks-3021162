fn third_party_api(username: &str, password: &str) -> String {
    /* Makes an outbound request to connet to the
     * server
     */

    let mut access_token = username.to_string();
    access_token.push_str(password.split_at(2).0);

    access_token
}

fn main() {
    // let username = "Elton".to_string();
    // let password = "my_password".to_string();
    let username = env!("USERNAME");
    // let username = option_env!("USERNAME").expect("'USERNAME' environment variable should be set!");
    let password = option_env!("PASSWORD").expect("'PASSWORD' environment variable should be set!");
    println!("username: '{}', password: '{}'", &username, &password);

    println!("Access Token: {}", third_party_api(&username, &password))
}
