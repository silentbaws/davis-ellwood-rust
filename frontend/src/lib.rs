mod common;
mod pages;

use std::collections::HashMap;

use common::{Header, NotFound};
use pages::{
    work::{WorkHome, WorkPage},
    HomePage,
};
use yew::prelude::*;
use yew_router::{
    history::{AnyHistory, History, MemoryHistory},
    BrowserRouter, Routable, Router, Switch,
};

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
        MainRoute::WorkRoot => html! {
            <Suspense>
                <WorkHome />
            </Suspense>
        },
        MainRoute::Work { id } => html! { <WorkPage id={id} /> },
        MainRoute::NotFound => html! { <NotFound /> },
        _ => html! { <h2 class="text-center"> {"under construction"} </h2> },
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
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Header />
            <Switch<MainRoute> render={switch} />
        </Router>
    }
}
