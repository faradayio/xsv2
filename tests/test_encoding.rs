use crate::workdir::Workdir;

fn data() -> Vec<Vec<String>> {
    vec![
        svec!["Email", "FirstName", "LastName", "Age"],
        svec!["alice@example.com", "Alice", "Smith", "30"],
        svec!["bob@example.com", "Bob", "Jones", "25"],
        svec!["charlie@example.com", "Charlie", "Brown", "35"],
    ]
}

#[test]
fn utf16le_headers() {
    let wrk = Workdir::new("utf16le_headers");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("headers");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
1   Email
2   FirstName
3   LastName
4   Age";
    assert_eq!(got, expected);
}

#[test]
fn utf16le_headers_just_names() {
    let wrk = Workdir::new("utf16le_headers_just_names");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("headers");
    cmd.arg("--just-names").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
Email
FirstName
LastName
Age";
    assert_eq!(got, expected);
}

#[test]
fn utf16le_select_by_name() {
    let wrk = Workdir::new("utf16le_select_by_name");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("select");
    cmd.arg("Email,FirstName").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email", "FirstName"],
        svec!["alice@example.com", "Alice"],
        svec!["bob@example.com", "Bob"],
        svec!["charlie@example.com", "Charlie"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn utf16le_select_by_index() {
    let wrk = Workdir::new("utf16le_select_by_index");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("select");
    cmd.arg("1,3").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email", "LastName"],
        svec!["alice@example.com", "Smith"],
        svec!["bob@example.com", "Jones"],
        svec!["charlie@example.com", "Brown"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn utf16le_count() {
    let wrk = Workdir::new("utf16le_count");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("count");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "3");
}

#[test]
fn utf16le_search() {
    let wrk = Workdir::new("utf16le_search");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("search");
    cmd.arg("Alice").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email", "FirstName", "LastName", "Age"],
        svec!["alice@example.com", "Alice", "Smith", "30"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn utf16le_search_select() {
    let wrk = Workdir::new("utf16le_search_select");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("search");
    cmd.arg("--select")
        .arg("FirstName")
        .arg("Bob")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email", "FirstName", "LastName", "Age"],
        svec!["bob@example.com", "Bob", "Jones", "25"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn utf16le_stats() {
    let wrk = Workdir::new("utf16le_stats");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("stats");
    cmd.arg("--select").arg("Age").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0][0], "field");
    assert_eq!(got[1][0], "Age");
    assert_eq!(got[1][2], "90");
    assert_eq!(got[1][3], "25");
    assert_eq!(got[1][4], "35");
}

#[test]
fn utf16be_headers() {
    let wrk = Workdir::new("utf16be_headers");
    wrk.create_with_encoding("data.csv", data(), "utf-16be");

    let mut cmd = wrk.command("headers");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
1   Email
2   FirstName
3   LastName
4   Age";
    assert_eq!(got, expected);
}

#[test]
fn utf16be_select() {
    let wrk = Workdir::new("utf16be_select");
    wrk.create_with_encoding("data.csv", data(), "utf-16be");

    let mut cmd = wrk.command("select");
    cmd.arg("Email").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email"],
        svec!["alice@example.com"],
        svec!["bob@example.com"],
        svec!["charlie@example.com"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn utf16le_frequency() {
    let wrk = Workdir::new("utf16le_frequency");
    let data = vec![
        svec!["Color", "Size"],
        svec!["Red", "Small"],
        svec!["Blue", "Large"],
        svec!["Red", "Medium"],
        svec!["Red", "Small"],
    ];
    wrk.create_with_encoding("data.csv", data, "utf-16le");

    let mut cmd = wrk.command("frequency");
    cmd.arg("--select").arg("Color").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0], svec!["field", "value", "count"]);
    assert!(got.iter().any(|row| row[1] == "Red" && row[2] == "3"));
    assert!(got.iter().any(|row| row[1] == "Blue" && row[2] == "1"));
}

#[test]
fn utf16le_slice() {
    let wrk = Workdir::new("utf16le_slice");
    wrk.create_with_encoding("data.csv", data(), "utf-16le");

    let mut cmd = wrk.command("slice");
    cmd.arg("--start")
        .arg("1")
        .arg("--len")
        .arg("2")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Email", "FirstName", "LastName", "Age"],
        svec!["bob@example.com", "Bob", "Jones", "25"],
        svec!["charlie@example.com", "Charlie", "Brown", "35"],
    ];
    assert_eq!(got, expected);
}
