use maud::{ Markup, html };

/// Card component
/// @params style: Option<String>
/// @params content: Option<Markup>
/// @returns Markup
pub fn card(
    styles: Option<String>,
    card_header: Option<Markup>,
    card_content: Option<Markup>,
    card_footer: Option<Markup>
) -> Markup {
    html! {
        div .(format!("rounded-lg border bg-card text-card-foreground shadow-sm {}", styles.unwrap_or_default())) {
            (card_header.unwrap_or_default())
            (card_content.unwrap_or_default())
            (card_footer.unwrap_or_default())
        }
    }
}

/// Card header component
/// @params style: Option<String>
/// @params content: Option<Markup>
/// @returns Markup
pub fn card_header(
    styles: Option<String>,
    card_title: Option<Markup>,
    card_description: Option<Markup>
) -> Markup {
    html! {
        div .(format!("flex flex-col space-y-1.5 p-6, {}", styles.unwrap_or_default())) {
            (card_title.unwrap_or_default())
            (card_description.unwrap_or_default())
        }
    }
}

pub fn card_title(styles: Option<String>, content: Option<String>) -> Markup {
    html! {
        div .(format!("text-2xl font-semibold leading-none tracking-tight, {}", styles.unwrap_or_default())) {
            (content.unwrap_or_default())
        }
    }
}

/// Card description component
/// @params style: Option<String>
/// @params content: Option<Markup>
/// @returns Markup
pub fn card_description(styles: Option<String>, content: Option<String>) -> Markup {
    html! {
        div .(format!("text-sm text-muted-foreground {}", styles.unwrap_or_default())) {
            (content.unwrap_or_default())
        }
    }
}

/// Card content component
/// @params style: Option<String>
/// @params content: Option<Markup>
/// @returns Markup
pub fn card_content(styles: Option<String>, content: Option<Markup>) -> Markup {
    html! {
        div .(format!("p-6 pt-0 {}", styles.unwrap_or_default()))
        { (content.unwrap_or_default()) }
    }
}

/// Card footer component
/// @params style: Option<String>
/// @params content: Option<Markup>
/// @returns Markup
pub fn card_footer(styles: Option<String>, content: Option<Markup>) -> Markup {
    html! {
        div .(format!("flex items-center p-6 pt-0 {}", styles.unwrap_or_default()))
        { (content.unwrap_or_default()) }
    }
}
