use async_trait::async_trait;
use url::Url;

use crate::generic_protocol::{DirEntry, GenericFileSystem, Metadata};

use super::ObjectFS;


#[async_trait(?Send)]
impl GenericFileSystem for ObjectFS {
	
	async fn metadata<P: Into<Url>>(path: P) -> anyhow::Result<Metadata> {
		todo!()
	}

	async fn read<P: Into<Url>>(path: P) -> anyhow::Result<Vec<u8>> {
		todo!()
	}

	async fn read_to_string<P: Into<Url>>(path: P) -> anyhow::Result<String> {todo!()}
	async fn remove_dir<P: Into<Url>>(path: P) -> anyhow::Result<()> {todo!()}
	async fn remove_file<P: Into<Url>>(path: P) -> anyhow::Result<()> {todo!()}
	async fn rename<P: Into<Url>, Q: Into<Url>>(source: P, destination: Q) -> anyhow::Result<()> {todo!()}
	async fn read_dir<P: Into<Url>, I: Iterator<Item=DirEntry>>(path: P) -> anyhow::Result<I> {todo!()}
	async fn copy<P: Into<Url>, Q: Into<Url>>(source: P, destination: Q) -> anyhow::Result<()> {todo!()}
}