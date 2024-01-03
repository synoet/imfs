use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Location ({location:?}) doest not exist.")]
    LocationDoesNotExistError { location: String },
    #[error("Location ({location:?}) already exists")]
    LocationAlreadyExistsError { location: String },
}
