use std::io::Result;
use url::Url;

use crate::generic_protocol::{DirEntry, Metadata};

pub fn metadata<P: Into<Url>>(path: &P) -> Result<Metadata> {
	todo!()
}

pub fn read<P: Into<Url>>(path: &P) -> Result<Vec<u8>> {
	todo!()
}

pub fn read_to_string<P: Into<Url>>(path: &P) -> Result<String> {
	read(path).map(|bytes_vec|{
		String::from_utf8_lossy(&bytes_vec).to_string()
	})
}

pub fn remove<P: Into<Url>>(path: &P) -> Result<()> {
	todo!()
}

pub fn rename<P: Into<Url>, Q: Into<Url>>(source: &P, destination: &Q) -> Result<()> {
	todo!()
}

pub fn move_<P: Into<Url>, Q: Into<Url>>(source: &P, destination: &Q) -> Result<()> {
	rename(source, destination)
}

pub fn read_dir<P: Into<Url>, I: Iterator<Item=DirEntry>>(path: &P) -> Result<I> {
	todo!()
}

pub fn copy<P: Into<Url>, Q: Into<Url>>(source: &P, destination: &Q) -> Result<()> {
	todo!()
}
