use std::{error::Error, io};

pub fn handle_status_error(status: u16) -> Result<(), Box<dyn Error>> {
    if status != 200 {
        let bad_status_error = io::Error::new(io::ErrorKind::Other, "Bad status code: {}");
        return Err(Box::new(bad_status_error));
    }

    Ok(())
}
