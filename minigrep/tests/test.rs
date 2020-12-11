use minigrep;

#[test]
fn one_result() {
    let query = "Duct";
    let content = "\
Rust:
safe,fast,productive.
Pick three.";

    assert_eq!(
        vec!["safe,fast,productive."],
        minigrep::search(query, content)
    )
}

#[test]
fn one_sensitive() {
    let query = "Duct";
    let content = "\
Rust:
safe,fast,productive.
Pick three.";

    assert_eq!(
        vec!["safe,fast,productive."],
        minigrep::search_case_insensitive(query, content)
    )
}
