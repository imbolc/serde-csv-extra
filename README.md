[![License](https://img.shields.io/crates/l/serde-csv-extra.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/serde-csv-extra.svg)](https://crates.io/crates/serde-csv-extra)
[![Docs.rs](https://docs.rs/serde-csv-extra/badge.svg)](https://docs.rs/serde-csv-extra)

# serde-csv-extra

Csv-related serde addons

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Foo {
   #[serde(with = "serde_csv_extra::vec_num")]
   list: Vec<i32>,
   #[serde(with = "serde_csv_extra::vec_vec_num")]
   matrix: Vec<Vec<i32>>,
   #[serde(with = "serde_csv_extra::maybe_image_size")]
   image_size: Option<(u8, u16)>,
   #[serde(with = "serde_csv_extra::maybe_lat_lon")]
   geo: Option<(f32, f32)>,
}

let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(Vec::new());
wtr.serialize(
    Foo {
        list: vec![-1, 1],
        matrix: vec![vec![-1, 1], vec![1, -1]],
        image_size: Some((16, 1024)),
        geo: Some((84.99, -135.00)),
    }
).unwrap();
wtr.serialize(
    Foo {
        list: vec![],
        matrix: vec![],
        image_size: None,
        geo: None,
    }
).unwrap();
let s = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
assert_eq!(s, "-1_1,-1_1|1_-1,16x1024,84.99;-135\n,,,\n");
```

## Contributing

We appreciate all kinds of contributions, thank you!

### Note on README

The `README.md` file isn't meant to be changed directly. It instead generated from the crate's docs
by the [cargo-readme] command:

* Install the command if you don't have it: `cargo install cargo-readme`
* Change the crate-level docs in `src/lib.rs`, or wrapping text in `README.tpl`
* Apply the changes: `cargo readme > README.md`

If you have [rusty-hook] installed the changes will apply automatically on commit.

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme]: https://github.com/livioribeiro/cargo-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
