use yew::prelude::*;
use yew_bootstrap::component::{Container, Column, Row, Lead};
use yew_router::prelude::Link;

use crate::MainRoute;

#[function_component]
pub fn HomePage() -> Html {
    
    html! {
        <Container class={"mb-5"}>
            <div class="shadow p-3 bg-light rounded border">
                <Row class="px-0 px-md-3">
                    <Column class="text-center pb-4 pt-1" size={12} >
                        <h2>{"About Me"}</h2>
                    </Column>

                    <Column class="d-block d-md-none" size={2} />
                    <Column class={"px-4 px-md-2 mb-3 align-self-center text-center"} size={8} md={4}>
                        <img src={"/static/home/portrait.svg"} class={"img-fluid d-block rounded-circle shadow-lg border border-dark border-5 mx-auto"} />
                    </Column>
                    <Column size={2} class="d-block d-md-none" />
                    <Column size={12} md={8}>
                        <Lead>{"Hi! I'm Davis, a back-end developer and gameplay programmer."}</Lead>
                        <Lead>{"To me programming is all about learning new things, I'm always trying to expand my skillset to incorporate new technologies and disciplines."}</Lead>
                        <Lead>
                            {"Recently I've been learning about network code with my "}
                            <Link<MainRoute> to={MainRoute::Work{id: String::from("skaterxl")}}>{"SkaterXL Multiplayer Mod"}</Link<MainRoute>>
                            {", RESTful APIs to enable stat tracking and real time server lists for my mod, and nodejs to add "}
                            <a href="https://github.com/silentbaws/Voicemeeter-Twitch-Bot" target="_blank">{"voice effects "}</a>
                            {"to anyone's twitch stream."}
                        </Lead>
                        <Lead>
                            {"I'm currently looking for work, if you'd like to get in contact with me for a job opportunity you can do so "}
                            <a href="mailto:davis.ellwood.1998@gmail.com">{"here."}</a>
                        </Lead>
                    </Column>
                </Row>
            </div>
        </Container>
    }
}