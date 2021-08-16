mod cloud_filesystem;
mod gcs_filesystem;
mod fs_detector;
mod generic_protocol;
mod object_filesystem;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
