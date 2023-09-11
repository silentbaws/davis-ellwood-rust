use model::work::WorkItem;
use yew::{prelude::*, virtual_dom::VChild};
use yew_bootstrap::component::{Column, Container, Lead, Row};
use yew_router::prelude::Link;

use crate::MainRoute;

async fn fetch_all_work() -> Result<Vec<WorkItem>, ()> {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("http://localhost:8080/api/work").await;

    match resp {
        Ok(response) => {
            let deserialized = response.json::<Vec<WorkItem>>().await;
            match deserialized {
                Ok(result) => Ok(result),
                Err(_) => Err({}),
            }
        }
        Err(_) => Err({}),
    }
}

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
pub fn WorkHome() -> HtmlResult {
    let work_items = use_prepared_state!(
        async move |_| -> Vec<WorkItem> { fetch_all_work().await.unwrap_or(vec![]) },
        ()
    )?
    .unwrap();

    Ok(html! {
        <Container fluid={true}>
            <Row class="row-cols-1 row-cols-md-2 row-cols-lg-3 row-cols-xl-4 justify-content-center mb-5">
                {
                    (*work_items).iter().map(|item| {
                        work_item_to_card(item)
                    }).collect::<Vec<VChild<Column>>>()
                }
            </Row>
        </Container>
    })
}
