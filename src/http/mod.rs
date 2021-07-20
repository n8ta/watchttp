use ureq;

pub fn download_site(url: &str) -> Result<String, ureq::Error> {
    let resp = ureq::get(url).call()?;

    if resp.status() == 200 {
        let body = resp.into_string()?;
        Result::Ok(body)
    } else {
        Result::Err(ureq::Error::Status(resp.status(), resp))
    }
}