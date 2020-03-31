use actix_web::HttpRequest;
use askama::Error;

pub fn url_for(req: &HttpRequest, name: &str, args: &[String]) -> Result<String, Error> {
    Ok(req.url_for(name, args).unwrap().to_string())
}
