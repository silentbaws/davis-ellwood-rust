use yew::{prelude::*, virtual_dom::VChild};
use yew_bootstrap::component::{Container, Column, Row, Lead};
use yew_router::prelude::Link;

use crate::MainRoute;

#[derive(Clone)]
pub struct FeatureAndChallenge {
    pub feature: String,
    pub challenge: String,
}

#[derive(Clone)]
pub struct WorkItem {
    pub image_url: AttrValue,
    pub img_desc: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,
    pub id: String,
    pub technologies: Vec<String>,
    pub features_and_challenges: Vec<FeatureAndChallenge>,
}

impl WorkItem {
    fn to_child_column(&self) -> VChild<Column> {
        html_nested! {
            <Column class="my-2">
                <div class="card h-100">
                    <div class="card-header text-center">
                        <Link<MainRoute> to={MainRoute::Work{id: self.id.to_owned()}}><h5 class="card-title">{self.title.to_owned()}</h5></Link<MainRoute>>
                    </div>
                    <img class="card-img-top" src={self.image_url.to_owned()}/>
                    <div class="card-body">
                        <Lead class="card-text">{self.img_desc.to_owned()}</Lead>
                    </div>
                </div>
            </Column>
        }
    }
}

#[function_component]
pub fn WorkHome() -> Html {
    let item_1 = WorkItem{
        image_url: AttrValue::from("https://images.unsplash.com/photo-1575936123452-b67c3203c357?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8aW1hZ2V8ZW58MHx8MHx8fDA%3D&w=1000&q=80"),
        description: AttrValue::from("a fancy work description"),
        img_desc: AttrValue::from("a fancy image description"),
        title: AttrValue::from("A fancy title"),
        id: String::from("fancy"),
        technologies: vec![],
        features_and_challenges: vec![FeatureAndChallenge { feature: String::from(""), challenge: String::from("") }],
    };

    //https://funnelgraphic.com/wp-content/uploads/2023/01/4-3-aspect-ratio-calculator-image-and-video.png
    let item_2 = WorkItem{
        image_url: AttrValue::from("https://funnelgraphic.com/wp-content/uploads/2023/01/4-3-aspect-ratio-calculator-image-and-video.png"),
        description: AttrValue::from("a less fancy work description"),
        img_desc: AttrValue::from("a less fancy image description"),
        title: AttrValue::from("A not so fancy title"),
        id: String::from("fancy"),
        technologies: vec![],
        features_and_challenges: vec![FeatureAndChallenge { feature: String::from(""), challenge: String::from("") }],
    };

    let work_cards = use_state(|| vec![
        item_1.clone(), item_2.clone(), item_1.clone(), item_2.clone(), item_1.clone(), item_2.clone(), item_2.clone()
    ]);

    html! {
    <Container fluid={true}>
        <Row class="row-cols-1 row-cols-md-2 row-cols-lg-3 row-cols-xl-4 justify-content-center mb-5">
            {
                work_cards.iter().map(|card| card.to_child_column()).collect::<Vec<VChild<Column>>>()
            }
        </Row>
    </Container>
    }
}
