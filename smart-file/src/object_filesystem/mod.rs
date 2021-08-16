mod connection_metadata;
pub use connection_metadata::*;

mod from_url;
pub use from_url::*;

mod generic_filesystem;
pub use generic_filesystem::*;

use crate::generic_protocol::{ObjectStorageCredentials, ObjectStoragePathRef};

#[derive(PartialEq, PartialOrd, Debug, Default)]
pub struct ObjectFS {
	path_ref: ObjectStoragePathRef,
	credentials: ObjectStorageCredentials,
	connection_metadata: ConnectionMetadata
}