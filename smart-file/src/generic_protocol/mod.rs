mod constants;
pub use constants::*;

mod supported_protocols;
pub use supported_protocols::*;

mod dir_entry;
pub use dir_entry::*;

mod metadata;
pub use metadata::*;

use url::Url;

use async_trait::async_trait;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct GenericProtocol {
	prefix: String,
	bucket: Option<String>,
	path: Option<String>
}

#[derive(PartialEq, PartialOrd, Debug, Default)]
pub struct ObjectStorageCredentials {
	username: Option<String>,
	password: Option<String>,
}

impl From<Url> for ObjectStorageCredentials {
	
	fn from(url: Url) -> Self {
		
		let username = 
			if url.username().is_empty() {
				None
			} else {
				Some(url.username().into())
			};


		ObjectStorageCredentials {
			username,
			password: url.password().map(|u| u.to_string())
		}
	}
}

#[derive(PartialEq, PartialOrd, Debug, Default)]
pub struct ObjectStoragePathRef {
	pub bucket_name: String,
	pub path: String,
}


impl From<Url> for ObjectStoragePathRef {
	
	fn from(url: Url) -> Self {
		
		let bucket = url.host_str().unwrap_or_default().to_string();
		let path = url.path()[0..].to_string();
		
		ObjectStoragePathRef {
			bucket_name: bucket,
			path
		}
	}
}


#[async_trait(?Send)]
pub trait GenericFileSystem {
	async fn metadata<P: Into<Url>>(path: P) -> anyhow::Result<Metadata>;
	async fn read<P: Into<Url>>(path: P) -> anyhow::Result<Vec<u8>>;
	async fn read_to_string<P: Into<Url>>(path: P) -> anyhow::Result<String>;
	async fn remove_dir<P: Into<Url>>(path: P) -> anyhow::Result<()>;
	async fn remove_file<P: Into<Url>>(path: P) -> anyhow::Result<()>;
	async fn rename<P: Into<Url>, Q: Into<Url>>(source: P, destination: Q) -> anyhow::Result<()>;
	async fn read_dir<P: Into<Url>, I: Iterator<Item=DirEntry>>(path: P) -> anyhow::Result<I>;
	async fn copy<P: Into<Url>, Q: Into<Url>>(source: P, destination: Q) -> anyhow::Result<()>;
}