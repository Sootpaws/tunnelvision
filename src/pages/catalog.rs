use super::templates::template;
use crate::search::Search;
use axum::extract::{Query, State};
use axum::http::uri::{Parts, PathAndQuery, Uri};
use axum::response::{IntoResponse, Redirect, Response};
use upon::value;

pub async fn page(State(data): State<crate::data::Data>, uri: Uri) -> Response {
    // Silly dance to remove empty query parameters otherwise deserialization
    // gets sad
    let query = uri
        .path_and_query()
        .and_then(|paq| paq.query())
        .unwrap_or("");
    let mut modified = false;
    let parts = form_urlencoded::parse(query.as_bytes()).filter(|(_, val)| {
        if val.is_empty() {
            modified = true;
            false
        } else {
            true
        }
    });
    let combined = form_urlencoded::Serializer::new(String::from("?"))
        .extend_pairs(parts)
        .finish();
    let mut rebuilt = Parts::default();
    rebuilt.path_and_query = Some(PathAndQuery::from_maybe_shared(combined.clone()).unwrap());
    let rebuilt = Uri::from_parts(rebuilt).unwrap();
    // Redirect to remove empty parameters
    if modified {
        return Redirect::permanent(&format!("/catalog{combined}")).into_response();
    }
    // Actual page handling
    match Query::<Search>::try_from_uri(&rebuilt) {
        Ok(Query(mut search)) => {
            search.normalize();
            let mut tags = data.tags.iter().collect::<Vec<_>>();
            tags.sort_by_key(|(_, tag)| &tag.name);
            let mut artists = data.artists.iter().collect::<Vec<_>>();
            artists.sort_by_key(|(_, artist)| &artist.name);
            let murals = search.apply(&data);
            template(
                "catalog",
                value! {
                    tags: tags,
                    tag: data.tags.get(&search.tag),
                    artists: artists,
                    artist: data.artists.get(&search.artist),
                    murals: &murals,
                    no_results: murals.is_empty(),
                    search: &search,
                    tag_only: search.tag_only(),
                    artist_only: search.artist_only(),
                    year_only: search.year_only(),
                    detailed_search: search.detailed(),
                    page_info: value! {
                        page_forward: serde_qs::to_string(&search.forwards(data.murals.len())).unwrap(),
                        page_back: serde_qs::to_string(&search.back()).unwrap()
                    }
                },
            )
        }
        Err(e) => super::error::page(e.into()),
    }
}
