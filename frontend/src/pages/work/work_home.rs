use yew::{prelude::*, virtual_dom::VChild};
use yew_bootstrap::component::{Container, Column, Row, Lead};
use yew_router::prelude::Link;
use model::work::WorkItem;

use crate::MainRoute;

fn work_item_to_card(work_item: &WorkItem) -> VChild<Column> {
    html_nested! {
        <Column class="my-2">
            <div class="card h-100">
                <div class="card-header text-center">
                    <Link<MainRoute> to={MainRoute::Work{id: work_item.id.to_owned()}}>
                        <h5 class="card-title">{work_item.title.to_owned()}</h5>
                    </Link<MainRoute>>
                </div>
                <img class="card-img-top" src={work_item.image_url.to_owned()}/>
                <div class="card-body">
                    <Lead class="card-text">{work_item.img_desc.to_owned()}</Lead>
                </div>
            </div>
        </Column>
    }
}

#[function_component]
pub fn WorkHome() -> Html {
    let work_items = use_state(|| Vec::<WorkItem>::new());

    html! {
    <Container fluid={true}>
        <Row class="row-cols-1 row-cols-md-2 row-cols-lg-3 row-cols-xl-4 justify-content-center mb-5">
            {
                work_items.iter().map(|item| work_item_to_card(item)).collect::<Vec<VChild<Column>>>()
            }
        </Row>
    </Container>
    }
}
