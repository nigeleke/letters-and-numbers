use maud::{html, Markup, DOCTYPE};

pub fn base(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=3, maximum-scale=1, user-scalable=no" {}
                title { "Letters & Numbers" }

                link rel="favicon" href="../assets/favicon.icp" {}
                link rel="stylesheet" href="../assets/css/style.css" {}

                script src="../assets/js/htmx.min.js" {}
                script src="../assets/js/trace.js" {}
            }
            body {
                (content)
            }
        }
    }
}
