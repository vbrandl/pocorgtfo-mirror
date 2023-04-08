use std::{fmt, fs, path::PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sha2::Sha256;

const MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Month {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", MONTHS[*self as usize])
    }
}

#[derive(Serialize, Deserialize)]
pub struct Issue<'a> {
    volume: u8,
    year: u16,
    month: Month,
    description: &'a str,
    files: Files<'a>,
}

impl<'a> Issue<'a> {
    #[inline]
    pub fn into_files(self) -> Files<'a> {
        self.files
    }
}

impl<'a> fmt::Display for Issue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "<h3><a href=\"./files/{}\">POC||GTFO 0x{:02}</a></h3><ul><li>{} {}<li>{}<li>{}</ul>",
            self.files.inner()[0].0,
            self.volume,
            self.month,
            self.year,
            self.description,
            self.files
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct File<'a>(&'a str);

impl<'a> File<'a> {
    fn hash(&self) -> ::std::io::Result<HashedFile<'a>> {
        let mut path = PathBuf::new();
        path.push("files");
        path.push(self.0);
        let content = fs::read(path)?;
        // let mut reader = BufReader::new(StdFile::open(path)?);
        // let mut content = String::new();
        // reader.read_to_string(&mut content)?;
        let sha1 = format!("{:x}", Sha1::digest(&content));
        let sha256 = format!("{:x}", Sha256::digest(&content));
        Ok(HashedFile {
            name: self.0,
            sha1,
            sha256,
        })
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.0
    }
}

struct HashedFile<'a> {
    name: &'a str,
    sha1: String,
    sha256: String,
}

#[derive(Serialize, Deserialize)]
pub struct Files<'a>(#[serde(borrow)] Vec<File<'a>>);

impl<'a> IntoIterator for Files<'a> {
    type Item = File<'a>;
    type IntoIter = <Vec<File<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Files<'a> {
    #[inline]
    fn inner(&self) -> &[File<'a>] {
        &self.0
    }
}

impl<'a> fmt::Display for Files<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hashed: Vec<_> = self
            .inner()
            .iter()
            .map(File::hash)
            .map(Result::unwrap)
            .collect();
        write!(f, "<pre>")?;
        hashed.iter().fold(Ok(()), |acc, file| {
            acc.and(writeln!(
                f,
                "SHA1({}) = <a href=\"./files/{}\">{}</a>",
                file.sha1, file.name, file.name
            ))
        })?;
        writeln!(f)?;
        hashed.iter().fold(Ok(()), |acc, file| {
            acc.and(writeln!(
                f,
                "SHA256({}) = <a href=\"./files/{}\">{}</a>",
                file.sha256, file.name, file.name
            ))
        })?;
        write!(f, "</pre>")
    }
}

impl<'a> PartialEq for Issue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.volume == other.volume
    }
}

impl<'a> Eq for Issue<'a> {}

impl<'a> PartialOrd for Issue<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        Some(self.volume.cmp(&other.volume))
    }
}

impl<'a> Ord for Issue<'a> {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.volume.cmp(&other.volume)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Year<'a>(u16, #[serde(borrow)] Vec<Issue<'a>>);

impl<'a> IntoIterator for Year<'a> {
    type Item = Issue<'a>;
    type IntoIter = <Vec<Issue<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.1.into_iter()
    }
}

impl<'a> Year<'a> {
    #[inline]
    fn inner(&self) -> (u16, &[Issue<'a>]) {
        (self.0, &self.1)
    }
}

impl<'a> fmt::Display for Year<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (year, issues) = self.inner();
        write!(f, "<h2>{}</h2><ul>", year)?;
        issues
            .iter()
            .fold(Ok(()), |acc, v| acc.and(write!(f, "<li>{}", v)))?;
        write!(f, "</ul>")
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mirror<'a>(#[serde(borrow)] Vec<Year<'a>>);

impl<'a> Mirror<'a> {
    #[inline]
    fn inner(&self) -> &[Year<'a>] {
        &self.0
    }
}

impl<'a> IntoIterator for Mirror<'a> {
    type Item = Year<'a>;
    type IntoIter = <Vec<Year<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> fmt::Display for Mirror<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<!doctype html><html><meta charset=\"utf-8\"><h1>International Journal of Proof-of-Concept or Get The Fuck Out (PoC||GTFO)</h1>"
        )?;
        self.inner()
            .iter()
            .fold(Ok(()), |acc, v| acc.and(write!(f, "{}", v)))?;
        write!(f, "</html>")
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config<'a>(#[serde(borrow)] Vec<Issue<'a>>);

impl<'a> Config<'a> {
    pub fn transform(self) -> Mirror<'a> {
        Mirror(
            self.0
                .into_iter()
                .group_by(|i| i.year)
                .into_iter()
                .map(|(y, i)| Year(y, i.collect::<Vec<_>>()))
                .collect::<Vec<_>>(),
        )
    }

    pub fn sort(mut self) -> Self {
        self.0.sort();
        self
    }
}
