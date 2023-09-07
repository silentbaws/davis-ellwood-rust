use yew::prelude::*;
use yew_bootstrap::component::{Container, Row, Column};

#[function_component]
pub fn NotFound () -> Html {
    html! {
        <Container>
            <Row>
                <Column size={12}>
                    <h2 class="text-center">{"404 | Page Not Found"}</h2>
                </Column>
            </Row>
        </Container>
    }
}