//! Csv-related serde addons
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!    #[serde(with = "serde_csv_extra::vec_num")]
//!    list: Vec<i32>,
//!    #[serde(with = "serde_csv_extra::vec_vec_num")]
//!    matrix: Vec<Vec<i32>>,
//!    #[serde(with = "serde_csv_extra::maybe_image_size")]
//!    image_size: Option<(u8, u16)>,
//!    #[serde(with = "serde_csv_extra::maybe_lat_lon")]
//!    geo: Option<(f32, f32)>,
//! }
//!
//! let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(Vec::new());
//! wtr.serialize(
//!     Foo {
//!         list: vec![-1, 1],
//!         matrix: vec![vec![-1, 1], vec![1, -1]],
//!         image_size: Some((16, 1024)),
//!         geo: Some((84.99, -135.00)),
//!     }
//! ).unwrap();
//! wtr.serialize(
//!     Foo {
//!         list: vec![],
//!         matrix: vec![],
//!         image_size: None,
//!         geo: None,
//!     }
//! ).unwrap();
//! let s = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
//! assert_eq!(s, "-1_1,-1_1|1_-1,16x1024,84.99;-135\n,,,\n");
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// `&[-1, 1]` <--> `-1_1`
pub mod vec_num {
    use serde::{self, de::Error, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    /// Serializer
    pub fn serialize<S, T>(list: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        let s = list
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("_");
        serializer.serialize_str(&s)
    }

    /// Deserializer
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Display,
        <T as FromStr>::Err: Display,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Vec::new());
        }
        s.split('_')
            .map(|n| n.parse().map_err(Error::custom))
            .collect()
    }
}

/// `&[[vec![-1, 1], vec![1, -1]]` <--> `-1_1|1_-1`
pub mod vec_vec_num {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    /// Serializer
    pub fn serialize<S, T>(rows: &[Vec<T>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        let s = rows
            .iter()
            .map(|c| {
                c.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("_")
            })
            .collect::<Vec<_>>()
            .join("|");
        serializer.serialize_str(&s)
    }

    /// Deserializer
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<Vec<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Display,
        <T as FromStr>::Err: Display,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Vec::new());
        }

        let mut rows = Vec::new();
        for line in s.split('|') {
            let mut row = Vec::new();
            for col_str in line.split('_') {
                row.push(col_str.parse().map_err(serde::de::Error::custom)?)
            }
            rows.push(row);
        }

        Ok(rows)
    }
}

/// `Some((128, 64))` <--> `128x64`
pub mod maybe_image_size {
    use serde::{self, de::Error, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    /// Serializer
    pub fn serialize<S, W, H>(size: &Option<(W, H)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        W: Display,
        H: Display,
    {
        if let Some(size) = size {
            let s = format!("{}x{}", size.0, size.1);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_str("")
        }
    }

    /// Deserializer
    pub fn deserialize<'de, D, W, H>(deserializer: D) -> Result<Option<(W, H)>, D::Error>
    where
        D: Deserializer<'de>,
        W: FromStr + Display,
        <W as FromStr>::Err: Display,
        H: FromStr + Display,
        <H as FromStr>::Err: Display,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let (width, height) = s
            .split_once("x")
            .ok_or_else(|| Error::custom("bad image size format"))?;
        Ok(Some((
            width.parse().map_err(Error::custom)?,
            height.parse().map_err(Error::custom)?,
        )))
    }
}

/// `Some((84.99, -135.00))` <--> `84.99;-135.00`
pub mod maybe_lat_lon {
    use serde::{self, de::Error, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    /// Serializer
    pub fn serialize<S, T>(size: &Option<(T, T)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        if let Some(size) = size {
            let s = format!("{};{}", size.0, size.1);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_str("")
        }
    }

    /// Deserializer
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<(T, T)>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Display,
        <T as FromStr>::Err: Display,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let (lat, lon) = s
            .split_once(";")
            .ok_or_else(|| Error::custom("bad image size format"))?;
        Ok(Some((
            lat.parse().map_err(Error::custom)?,
            lon.parse().map_err(Error::custom)?,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn vec_i32() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Foo(#[serde(with = "vec_num")] Vec<i32>);

        let foo = Foo(vec![-1, 0, 3]);
        let foo_str = "\"-1_0_3\"";

        let serialized = serde_json::to_string(&foo).unwrap();
        assert_eq!(serialized, foo_str);

        let deserialized: Foo = serde_json::from_str(foo_str).unwrap();
        assert_eq!(deserialized, foo);

        let empty = Foo(vec![]);
        let empty_str = "\"\"";

        let serialized = serde_json::to_string(&empty).unwrap();
        assert_eq!(serialized, empty_str);

        let deserialized: Foo = serde_json::from_str(empty_str).unwrap();
        assert_eq!(deserialized, empty);
    }

    #[test]
    fn vec_vec_i32() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Foo(#[serde(with = "vec_vec_num")] Vec<Vec<i32>>);

        let foo = Foo(vec![vec![0], vec![-1, 1]]);
        let foo_str = "\"0|-1_1\"";

        let serialized = serde_json::to_string(&foo).unwrap();
        assert_eq!(serialized, foo_str);

        let deserialized: Foo = serde_json::from_str(foo_str).unwrap();
        assert_eq!(deserialized, foo);

        let empty = Foo(vec![]);
        let empty_str = "\"\"";

        let serialized = serde_json::to_string(&empty).unwrap();
        assert_eq!(serialized, empty_str);

        let deserialized: Foo = serde_json::from_str(empty_str).unwrap();
        assert_eq!(deserialized, empty);
    }

    #[test]
    fn img_size() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Foo(#[serde(with = "maybe_image_size")] Option<(u8, u16)>);

        let foo = Foo(Some((1, 2)));
        let foo_str = "\"1x2\"";

        let serialized = serde_json::to_string(&foo).unwrap();
        assert_eq!(serialized, foo_str);

        let deserialized: Foo = serde_json::from_str(foo_str).unwrap();
        assert_eq!(deserialized, foo);

        let empty = Foo(None);
        let empty_str = "\"\"";

        let serialized = serde_json::to_string(&empty).unwrap();
        assert_eq!(serialized, empty_str);

        let deserialized: Foo = serde_json::from_str(empty_str).unwrap();
        assert_eq!(deserialized, empty);
    }

    #[test]
    fn geo() {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        struct Foo(#[serde(with = "maybe_lat_lon")] Option<(f32, f32)>);

        let foo = Foo(Some((-1.1, 1.1)));
        let foo_str = "\"-1.1;1.1\"";

        let serialized = serde_json::to_string(&foo).unwrap();
        assert_eq!(serialized, foo_str);

        let deserialized: Foo = serde_json::from_str(foo_str).unwrap();
        assert_eq!(deserialized, foo);

        let empty = Foo(None);
        let empty_str = "\"\"";

        let serialized = serde_json::to_string(&empty).unwrap();
        assert_eq!(serialized, empty_str);

        let deserialized: Foo = serde_json::from_str(empty_str).unwrap();
        assert_eq!(deserialized, empty);
    }
}
