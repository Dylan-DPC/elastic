//Autogenerated
use hyper::Client;
pub fn get_repository(base: String, repository: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + repository.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/snapshots/");
    url_fmtd.push_str(&repository);
    url_fmtd
}