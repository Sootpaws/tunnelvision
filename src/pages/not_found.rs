use super::templates::template;
use axum::http::uri::Uri;
use axum::response::Response;
use upon::value;

pub async fn page(uri: Uri) -> Response {
    if !check_ignore(&uri) {
        eprintln!("Page not found: {uri}");
    }
    template("not_found", value! {})
}

fn check_ignore(uri: &Uri) -> bool {
    IGNORED_PATH_PREFIXES.iter().any(|prefix| uri.path().starts_with(prefix)) ||
        IGNORED_PATH_SUFFIXES.iter().any(|prefix| uri.path().ends_with(prefix)) ||
        IGNORED_PATHS.contains(&uri.path())
}

const IGNORED_PATH_PREFIXES: [&str; 5] = [
    "/.",
    "/administrator",
    "/php",
    "/plugins",
    "/wp-",
];

const IGNORED_PATH_SUFFIXES: [&str; 1] = [
    ".php",
];

const IGNORED_PATHS: [&str; 31] = [
    "/.boto",
    "/.s3cfg",
    "/_pma/",
    "/account/",
    "/apple-touch-icon.png",
    "/apple-touch-icon-precomposed.png",
    "/db/",
    "/dbadmin/",
    "/Dockerfile",
    "/edit-account/",
    "/edit-profile/",
    "/enhancecp",
    "/favicon.ico",
    "/favicon.png",
    "/members/",
    "/my-account/",
    "/my-account/edit-account/",
    "/my-profile/",
    "/mysqladmin/",
    "/pma/",
    "/profile/",
    "/readme.html",
    "/register/",
    "/registration/",
    "/robots.txt",
    "/user/",
    "/user-profile/",
    "/sign-up/",
    "/signup/",
    "/sitemap.xml",
    "/sql/",
];
