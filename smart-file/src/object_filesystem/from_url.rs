use url::Url;

use crate::generic_protocol::{ObjectStorageCredentials, ObjectStoragePathRef};

use super::ObjectFS;

impl From<Url> for ObjectFS {
    
	fn from(url: Url) -> Self {	
		let credentials = ObjectStorageCredentials::from(url.clone());
		let path_ref = ObjectStoragePathRef::from(url.clone());
		
		ObjectFS { path_ref, credentials, ..ObjectFS::default() }
  }
}

#[cfg(test)]
mod tests {

    use url::Url;

    use crate::{generic_protocol::{ObjectStorageCredentials, ObjectStoragePathRef}, object_filesystem::ObjectFS};

    #[test]
    fn should_parse_url_with_folder_into_s3_fs() {
			let password = "password".to_string();
			let username = "username".to_string();
			
			let bucket = "bucket".to_string();
			let path = "path/to/file.test".to_string();

			let string_url = format!("s3://{}:{}@{}/{}", username, password, bucket, path);
			let url = Url::parse(&string_url).unwrap();

			let credentials: ObjectStorageCredentials = url.clone().into();
			let path_ref = ObjectStoragePathRef::from(url.clone());
			
      let expected = ObjectFS {path_ref, credentials, ..ObjectFS::default()};
			let result = ObjectFS::from(url);

			assert_eq!(expected, result);
    }

		#[test]
		fn should_parse_url_without_folder_into_s3_fs() {
			let password = "password".to_string();
			let username = "username".to_string();
			
			let bucket = "bucket".to_string();
			let path = "file.test".to_string();

			let string_url = format!("s3://{}:{}@{}/{}", username, password, bucket, path);
			let url = Url::parse(&string_url).unwrap();

			let credentials: ObjectStorageCredentials = url.clone().into();
			let path_ref = ObjectStoragePathRef::from(url.clone());

      let expected = ObjectFS {path_ref, credentials, ..ObjectFS::default()};
			let result = ObjectFS::from(url);

			assert_eq!(expected, result);
    }
}
