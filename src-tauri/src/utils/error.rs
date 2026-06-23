use anyhow::Result;

pub type GsResult<T> = Result<T>;

// pub use anyhow::{anyhow, bail, ensure};

pub fn log_error(err: &anyhow::Error) {
    log::error!("{:?}", err);
}

pub trait ToTauriError {
    fn to_tauri_error(self) -> String;
}

impl<E> ToTauriError for E
where
    E: Into<anyhow::Error>,
{
    fn to_tauri_error(self) -> String {
        let err = self.into();
        log_error(&err);
        err.to_string()
    }
}