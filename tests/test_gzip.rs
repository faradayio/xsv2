use std::fs::File;
use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::workdir::Workdir;

/// Helper function to create a gzipped CSV file
fn create_gzipped_csv(wrk: &Workdir, filename: &str, content: &str) {
    let path = wrk.path(filename);
    let file = File::create(path).unwrap();
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(content.as_bytes()).unwrap();
    encoder.finish().unwrap();
}

#[test]
fn gzip_count() {
    let wrk = Workdir::new("gzip_count");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("count");
    cmd.arg("data.csv.gz");
    let count: usize = wrk.stdout(&mut cmd);
    assert_eq!(count, 3);
}

#[test]
fn gzip_select() {
    let wrk = Workdir::new("gzip_select");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("select");
    cmd.arg("name,city").arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "city"],
        svec!["Alice", "NYC"],
        svec!["Bob", "LA"],
        svec!["Charlie", "Chicago"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn gzip_stats() {
    let wrk = Workdir::new("gzip_stats");
    let csv_content = "name,age\nAlice,30\nBob,25\nCharlie,35\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Check that we got stats output with the expected fields
    assert_eq!(
        got[0],
        svec![
            "field",
            "type",
            "sum",
            "min",
            "max",
            "min_length",
            "max_length",
            "mean",
            "stddev"
        ]
    );
    assert_eq!(got[1][0], "name");
    assert_eq!(got[2][0], "age");
    assert_eq!(got[2][1], "Integer");
}

#[test]
fn gzip_search() {
    let wrk = Workdir::new("gzip_search");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("search");
    cmd.arg("NYC").arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "age", "city"], svec!["Alice", "30", "NYC"]];
    assert_eq!(got, expected);
}

#[test]
fn gzip_headers() {
    let wrk = Workdir::new("gzip_headers");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("headers");
    cmd.arg("data.csv.gz");

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("name"));
    assert!(got.contains("age"));
    assert!(got.contains("city"));
}

#[test]
fn gzip_tsv() {
    let wrk = Workdir::new("gzip_tsv");
    let tsv_content = "name\tage\tcity\nAlice\t30\tNYC\nBob\t25\tLA\n";
    create_gzipped_csv(&wrk, "data.tsv.gz", tsv_content);

    let mut cmd = wrk.command("count");
    cmd.arg("data.tsv.gz");
    let count: usize = wrk.stdout(&mut cmd);
    assert_eq!(count, 2);
}

#[test]
fn gzip_slice() {
    let wrk = Workdir::new("gzip_slice");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\nDiana,28,SF\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("slice");
    cmd.arg("--start").arg("1").arg("--len").arg("2");
    cmd.arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["Bob", "25", "LA"],
        svec!["Charlie", "35", "Chicago"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn gzip_sort() {
    let wrk = Workdir::new("gzip_sort");
    let csv_content = "name,age\nCharlie,35\nAlice,30\nBob,25\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("sort");
    cmd.arg("--select").arg("name");
    cmd.arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age"],
        svec!["Alice", "30"],
        svec!["Bob", "25"],
        svec!["Charlie", "35"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn gzip_frequency() {
    let wrk = Workdir::new("gzip_frequency");
    let csv_content = "city\nNYC\nLA\nNYC\nChicago\nNYC\n";
    create_gzipped_csv(&wrk, "data.csv.gz", csv_content);

    let mut cmd = wrk.command("frequency");
    cmd.arg("--select").arg("city");
    cmd.arg("data.csv.gz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Check that we got frequency output
    assert_eq!(got[0], svec!["field", "value", "count"]);
    // NYC should appear 3 times
    assert!(got
        .iter()
        .any(|row| row.len() >= 3 && row[1] == "NYC" && row[2] == "3"));
}
