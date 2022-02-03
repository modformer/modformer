use std::env;

use anyhow::{
    Error,
    Result,
};

pub(crate) fn get_exec_name() -> Result<String> {
    env::current_exe()
        .map_err(Error::from)
        .and_then(|pbuf| {
            pbuf.file_name()
                .map(|ostr| ostr.to_os_string())
                .ok_or(Error::msg("File name not obtained"))
        })
        .and_then(|ostr| {
            ostr.into_string()
                .map_err(|_| Error::msg("OsString unconvertable"))
        })
}
