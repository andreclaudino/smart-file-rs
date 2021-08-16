use url::Url;

#[derive(Debug)]
pub enum SupportedProtocols { S3, Gs, Az, File, Other(String) }

impl From<Url> for SupportedProtocols {
    
	fn from(url: Url) -> Self {
		match url.scheme() {
			S3_PREFFIX => SupportedProtocols::S3,
			GS_PREFFIX => SupportedProtocols::Gs,
			AZ_PREFFIX => SupportedProtocols::Az,
			FILE_PREFFIX => SupportedProtocols::File,
			protocol => SupportedProtocols::Other(protocol.to_string())
		}		
  }
}