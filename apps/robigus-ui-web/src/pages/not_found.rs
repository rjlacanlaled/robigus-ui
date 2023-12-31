use actix_web::Responder;
use maud::{ html, PreEscaped };

use crate::strings;

pub async fn not_found() -> impl Responder {
    (
        html! {
            html lang="en" {
                head {
                    meta charset=(strings::UTF8);
                    title { (strings::NOT_FOUND_TITLE) }
                    meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                    style { (strings::NOT_FOUND_STYLE) }
                }
                body {
                    h1 { (strings::NOT_FOUND_TITLE) }
                    p { (strings::NOT_FOUND_MESSAGE) }
                }
                (PreEscaped(strings::NOT_FOUND_COMMENT))
            }
        },
        actix_web::http::StatusCode::NOT_FOUND,
    )
}
