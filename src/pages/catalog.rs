use super::templates::template;
use crate::search::Search;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::http::uri::{Uri, Parts, PathAndQuery};
use upon::value;

pub async fn page(State(data): State<crate::data::Data>, uri: Uri) -> Response {
    // Silly dance to remove empty query parameters otherwise deserialization
    // gets sad
    let query = uri.path_and_query()
        .and_then(|paq| paq.query())
        .unwrap_or("");
    let parts = form_urlencoded::parse(query.as_bytes())
        .filter(|(_, val)| !val.is_empty());
    let combined = form_urlencoded::Serializer::new(String::from("?")).extend_pairs(parts).finish();
    let mut rebuilt = Parts::default();
    rebuilt.path_and_query = Some(PathAndQuery::from_maybe_shared(combined).unwrap());
    let rebuilt = Uri::from_parts(rebuilt).unwrap();
    // Actual page handling
    let Query(search): Query<Search> = Query::try_from_uri(&rebuilt).unwrap();
    let murals = search.apply(&data.murals);
    template(
        "catalog",
        value! {
            murals: murals,
            search: &search,
            no_search: search.is_empty()
        },
    )
}
