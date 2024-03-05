use maud::{html, Markup, PreEscaped};
use sbm::{Bookmark, Category, Header, Sbm};

pub fn bookmark (bookmark: &Bookmark) -> Markup {
    html! {
        li {
            a href=(bookmark.url) {
                (bookmark.name)
            }
            " - "
            (bookmark.description)
        }
    }
}

pub fn category (category: &Category) -> Markup {
    html! {
        h2 {
            (category.header.name)
            @if let Some(icon) = &category.header.icon {
                (PreEscaped(icon))
            }
        }
        ul {
            @for bm in &category.bookmarks {
                (bookmark(bm))
            }
        }
    }
}

pub fn sbm (sbm: &Sbm) -> Markup {
    html! {
        @for cat in sbm.categories() {
            (category(cat))
        }
    }
}

pub fn render (cats: &Sbm) -> String {
    html! {
        html {
            head {
                title { "Bookmarks" }
            }
            body {
                (sbm(cats))
            }
        }
    }
    .into_string()
}