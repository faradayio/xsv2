use std::fs::File;
use std::io::Write;

use crate::workdir::Workdir;

/// Helper function to create a zstd-compressed CSV file
fn create_zstd_csv(wrk: &Workdir, filename: &str, content: &str) {
    let path = wrk.path(filename);
    let file = File::create(path).unwrap();
    let mut encoder = zstd::stream::write::Encoder::new(file, 3).unwrap();
    encoder.write_all(content.as_bytes()).unwrap();
    encoder.finish().unwrap();
}

#[test]
fn zstd_count() {
    let wrk = Workdir::new("zstd_count");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("count");
    cmd.arg("data.csv.zst");
    let count: usize = wrk.stdout(&mut cmd);
    assert_eq!(count, 3);
}

#[test]
fn zstd_select() {
    let wrk = Workdir::new("zstd_select");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("select");
    cmd.arg("name,city").arg("data.csv.zst");

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
fn zstd_stats() {
    let wrk = Workdir::new("zstd_stats");
    let csv_content = "name,age\nAlice,30\nBob,25\nCharlie,35\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv.zst");

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
fn zstd_search() {
    let wrk = Workdir::new("zstd_search");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("search");
    cmd.arg("NYC").arg("data.csv.zst");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "age", "city"], svec!["Alice", "30", "NYC"]];
    assert_eq!(got, expected);
}

#[test]
fn zstd_headers() {
    let wrk = Workdir::new("zstd_headers");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("headers");
    cmd.arg("data.csv.zst");

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("name"));
    assert!(got.contains("age"));
    assert!(got.contains("city"));
}

#[test]
fn zstd_tsv() {
    let wrk = Workdir::new("zstd_tsv");
    let tsv_content = "name\tage\tcity\nAlice\t30\tNYC\nBob\t25\tLA\n";
    create_zstd_csv(&wrk, "data.tsv.zst", tsv_content);

    let mut cmd = wrk.command("count");
    cmd.arg("data.tsv.zst");
    let count: usize = wrk.stdout(&mut cmd);
    assert_eq!(count, 2);
}

#[test]
fn zstd_slice() {
    let wrk = Workdir::new("zstd_slice");
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago\nDiana,28,SF\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("slice");
    cmd.arg("--start").arg("1").arg("--len").arg("2");
    cmd.arg("data.csv.zst");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["Bob", "25", "LA"],
        svec!["Charlie", "35", "Chicago"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn zstd_sort() {
    let wrk = Workdir::new("zstd_sort");
    let csv_content = "name,age\nCharlie,35\nAlice,30\nBob,25\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("sort");
    cmd.arg("--select").arg("name");
    cmd.arg("data.csv.zst");

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
fn zstd_frequency() {
    let wrk = Workdir::new("zstd_frequency");
    let csv_content = "city\nNYC\nLA\nNYC\nChicago\nNYC\n";
    create_zstd_csv(&wrk, "data.csv.zst", csv_content);

    let mut cmd = wrk.command("frequency");
    cmd.arg("--select").arg("city");
    cmd.arg("data.csv.zst");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Check that we got frequency output
    assert_eq!(got[0], svec!["field", "value", "count"]);
    // NYC should appear 3 times
    assert!(got
        .iter()
        .any(|row| row.len() >= 3 && row[1] == "NYC" && row[2] == "3"));
}

#[test]
fn compress_fixlengths_zstd() {
    let wrk = Workdir::new("compress_fixlengths_zstd");
    let csv_data = "a,b\n1\n2,3,4\n";
    std::fs::write(wrk.path("data.csv"), csv_data).unwrap();

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("--compress").arg("zstd");
    cmd.arg("--output").arg("out.csv.zst");
    cmd.arg("data.csv");
    wrk.run(&mut cmd);

    // Verify output is compressed and readable with proper fixed lengths
    let mut cmd2 = wrk.command("select");
    cmd2.arg("1-").arg("out.csv.zst");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got.len(), 3);
    assert_eq!(got[0].len(), 3);
    assert_eq!(got[1].len(), 3);
    assert_eq!(got[2].len(), 3);
}

#[test]
fn compress_split_zstd() {
    let wrk = Workdir::new("compress_split_zstd");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "30"],
            svec!["Bob", "25"],
            svec!["Charlie", "35"],
            svec!["Diana", "28"],
        ],
    );

    let mut cmd = wrk.command("split");
    cmd.arg("--compress").arg("zstd");
    cmd.arg("--size").arg("2");
    cmd.arg("--filename").arg("{}.csv.zst");
    cmd.arg("outdir");
    cmd.arg("data.csv");
    wrk.run(&mut cmd);

    // Verify the split files are compressed and readable
    let mut cmd2 = wrk.command("count");
    cmd2.arg("outdir/0.csv.zst");
    let count1: usize = wrk.stdout(&mut cmd2);
    assert_eq!(count1, 2);

    let mut cmd3 = wrk.command("count");
    cmd3.arg("outdir/2.csv.zst");
    let count2: usize = wrk.stdout(&mut cmd3);
    assert_eq!(count2, 2);
}
