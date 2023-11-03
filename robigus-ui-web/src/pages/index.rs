use actix_web::HttpRequest;
use maud::{ Markup, html };

use super::base::page;

pub async fn index(req: HttpRequest, child_content: Markup, title: &str, desc: &str) -> Markup {
    let host = format!("{}", req.uri());
    let lang = "en";
    let content =
        html! {
        #content class = "min-h-screen font-sans antialiased grainy bg-gradient-to-r from-gray-100 to-gray-30" {
            (child_content)
        }
    };
    page(&host, title, desc, lang, content)
}
