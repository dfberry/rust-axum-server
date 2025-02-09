use axum::{
    response::Html,
    extract::Extension,
};
use std::sync::Arc;
use crate::state::AppState;

/*

<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
<link rel="manifest" href="/site.webmanifest">

*/


pub async fn root_get_handler(
    Extension(state): Extension<Arc<AppState>>
) -> Html<String> {

    let version = state.config.read().unwrap().package.version.clone();

        let html_content = format!(
            "<h1>Source board</h1> <p>Version: {version}</p>"
        );

    Html(html_content)
}