use actix_web::{ HttpRequest, Result as ActixResult, get };
use maud::{ Markup, html };

use crate::pages::index::index;
use robigus_ui::components::card;

#[get("/")]
pub async fn page(req: HttpRequest) -> ActixResult<Markup> {
    let title: &str = "Home";
    let desc = "Robigus UI Web";

    // Card Header
    let card_title = card::card_title(None, Some("Card Title".to_owned()));
    let card_desc = card::card_description(None, Some("Card Description".to_owned()));
    let card_header = card::card_header(None, Some(card_title), Some(card_desc));

    // Card Content
    let card_content =
        html! {
        div {
            p { "Card Content" }
            p { "Card Content 2" }
        }
    };

    let card_footer = html! {
        div {
            p { "Card Footer" }
        }
    };

    let content =
        html! {
        div {
            ("card")
            (card::card(None, Some(card_header), Some(card_content), Some(card_footer)))
        }
    };

    Ok(index(req, content, title, desc).await)
}
