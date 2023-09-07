mod common;
mod pages;

use std::collections::HashMap;

use common::{Header, NotFound};
use pages::{HomePage, work::{WorkHome, WorkPage}};
use yew::prelude::*;
use yew_router::{Router, history::{AnyHistory, MemoryHistory, History}, Routable, BrowserRouter, Switch};

// // Example function to fetch a UUID from a web api -> works in browser and in ssr
// async fn fetch_uuid() {
//     // reqwest works for both non-wasm and wasm targets.
//     let resp = reqwest::get("https://httpbin.org/uuid").await;
//     match resp {
//         Ok(resp) => println!("{:#?}", resp),
//         Err(err) => println!("{}", err),
//     }

//     // how you would get the uuid in HtmlResult function
//     // let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())?.unwrap();
// }

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/work")]
    WorkRoot,
    #[at("/work/:id")]
    Work { id: String },
    #[at("/blog")]
    Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum WorkRoute {
    #[at("/work")]
    AllWork,
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! { <HomePage /> },
        MainRoute::WorkRoot => html! { <WorkHome /> },
        MainRoute::Work { id } => html! { <WorkPage id={id} /> },
        MainRoute::NotFound => html! { <NotFound /> },
        _ => html! { <h2 class="text-center"> {"under construction"} </h2> }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<MainRoute> render={switch} />
        </BrowserRouter>
    }
}

#[function_component]
pub fn ServerApp(props: &AppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push_with_query(&*props.url, &props.queries).unwrap();

    html! {
        <Router history={history}>
            <Header />
            <Switch<MainRoute> render={switch} />
        </Router>
    }
}