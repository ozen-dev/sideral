use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect},
};

pub async fn summon(headers: HeaderMap) -> impl IntoResponse {
    let user_agent_value = headers.get("User-Agent").unwrap();
    let user_agent = user_agent_value.to_str().unwrap();

    if user_agent.contains("curl") {
        Html(
            r#"#!/bin/bash
            open_url="https://cal.com/sideral"
            if which xdg-open > /dev/null; then
                xdg-open "$open_url"
            elif which gnome-open > /dev/null; then
                gnome-open "$open_url"
            elif which open > /dev/null; then
                open "$open_url"
            else
                echo "Could not detect the web browser to use. Open URL manually: https://cal.com/sideral"
            fi
            "#,
        )
        .into_response()
    } else {
        Redirect::temporary("https://sideral.design").into_response()
    }
}
