use yew::prelude::*;
use yew_bootstrap::{component::{NavBar, BrandType, NavDropdownItem, NavItem, Alert}, util::Dimension};
use yew_hooks::use_effect_once;
use yew_router::prelude::{use_location, Location};

#[function_component]
pub fn Header() -> Html {
    // I used to have some functionality around this where I would fade it at the top of the page
    // I've removed that now becuase I don't like it anymore
    // However, to remember how to implement it I'm leaving this note
    // In the server renderer I render out these classes
    // In the client renderer I have a HeaderWrapper that just calls the Header with props
    // The Header takes scroll and window size properties from the HeaderWrapper
    // The server passes defaults of 0 for all props.
    // I use yew_hooks use_window_size() and use_window_scroll() in the wrapper
    // These hooks force rerenders and pass the properties to the header
    // Without the wrapper the hooks cause the server render to fail because wasm isn't bound
    const BASE_NAVBAR_CLASSES: &str = "navbar-expand-lg navbar-light bg-light";

    let brand: BrandType = BrandType::BrandCombined {
        image_url: AttrValue::from("/static/header/brand-portrait.webp"),
        text: AttrValue::from(""),
        url: Some(AttrValue::from("/")),
        dimension: Some(Dimension{
            height: String::from("32px"),
            width: String::from("32px")
        }),
        alt: AttrValue::from("DE")
    };

    let location = use_location();
    let curr_path = location_to_path(&location).to_lowercase();

    let js_enabled = use_state(|| false);

    {
        let js_enabled = js_enabled.clone();

        use_effect_once(move || {
            js_enabled.set(true);
            || {}
        });
    }

    html! {
        <>
            <div class="fixed-top">
                <NavBar nav_id={"main_navbar"} class={ Classes::from(BASE_NAVBAR_CLASSES) } brand={brand}>
                    <NavItem text={"Home"} url={"/"} active={curr_path == "/"} />
                    <NavItem text={"My Work"} active={curr_path.starts_with("/work")}>
                        <NavDropdownItem url={"/work"} text="All Work"/>
                        <div class="dropdown-divider"></div>
                        <NavDropdownItem url={"/work/skaterxl"} text="Skater XL Multiplayer Mod" />
                        <NavDropdownItem url={"/work/blades"} text="Blades of Orterra" />
                        <NavDropdownItem url={"/work/website"} text="My Website" />
                    </NavItem>
                    <NavItem text={"Blog"} url={"/blog"} active={curr_path.starts_with("/blog")}/>
                </NavBar>
                {
                    if !*js_enabled {
                        html! { <Alert class="text-center">{"This page may not function as intended without the use of javascript/webassembly"}</Alert> }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div class="header-image"><div class="header-text-center"><h1 class="header-text">{"Davis Ellwood"}</h1></div></div>
        </>
    }
}

fn location_to_path(location: &Option<Location>) -> String {
    match location {
        Some(value) => value.path().to_owned(),
        None => "/".to_owned(),
    }
}