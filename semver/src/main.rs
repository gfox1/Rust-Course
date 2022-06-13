use std::fmt::Display;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
struct SemVer {
    major: u8,
    minor: u8,
    patch: u8
}

impl SemVer {
    fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self { major, minor, patch }
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<Vec<u8>> for SemVer {
    fn from(vector: Vec<u8>) -> Self {
        assert_eq!(vector.len(), 3);
        Self { major: vector[0], minor: vector[1], patch: vector[2] }
    }
}

#[derive(Debug)]
struct ErrString(String);

impl Display for ErrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ErrString {
    fn err() -> Box<Self> {
        Box::new(ErrString("Some error".to_string()))
    }
}

impl Error for ErrString {}

type DynError = Box<dyn Error>;

impl TryFrom<&str> for SemVer {
    type Error = DynError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(".").collect();

        if parts.len() != 3 {
            return Err(ErrString::err());
        }

        let mut parts = parts.into_iter().map(|e| e.parse());
        let major = parts.next().unwrap()?;
        let minor = parts.next().unwrap()?;
        let patch = parts.next().unwrap()?;

        Ok(SemVer {
            major,
            minor,
            patch,
        })
    }
}


fn main() -> Result<(), DynError> {
    //let sem = SemVer { major: 0, minor: 1, patch: 0};
    //let sem = SemVer::new(0, 1, 0);
    //let v: Vec<u8> = vec![1, 0, 0];
    //let sem = SemVer::from(v);
    //let _sem2: SemVer = vec![1, 1, 1].into();
    //let sem_str = SemVer::from("4.0.3");

    let sem_result = SemVer::try_from("5.0.1")?;
    
    println!("{}", sem_result);
    Ok(())
}

#[test]
fn test_semver_new() {
    assert_eq!(SemVer::new(0, 1, 0), SemVer { major: 0, minor: 1, patch: 0});
}

#[test]
fn test_semver_from_vecu8() {
    let v: Vec<u8> = vec![1, 0, 0];
    let sem = SemVer::from(v);
    let sem2: SemVer = vec![1, 1, 1].into();
    assert_eq!(sem, SemVer { major: 1, minor: 0, patch: 0});
    assert_eq!(sem2, SemVer { major: 1, minor: 1, patch: 1});
}

#[test]
fn test_semver_from_str() {
    let sem_str = SemVer::from("4.0.3");
    assert_eq!(sem_str, SemVer { major: 4, minor: 0, patch: 3});
}