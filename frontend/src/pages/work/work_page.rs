use yew::{prelude::*, virtual_dom::VChild};
use yew_bootstrap::component::{Container, Row, Column, Lead, AccordionItem, Accordion};

use model::work::{WorkItem, FeatureItem};

#[derive(Clone, Properties, PartialEq)]
pub struct WorkPageProps {
    #[prop_or(String::from(""))]
    pub id: String
}

#[function_component]
pub fn WorkPage (_props: &WorkPageProps) -> Html {
    let item_1 = WorkItem{
        image_url: String::from("https://images.unsplash.com/photo-1575936123452-b67c3203c357?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8aW1hZ2V8ZW58MHx8MHx8fDA%3D&w=1000&q=80"),
        description: String::from("a fancy work description"),
        img_desc: String::from("a fancy image description"),
        title: String::from("A fancy title"),
        id: String::from("fancy"),
        technologies: vec![
            String::from("C#"),
            String::from("C# Reflection"),
            String::from("Json"),
            String::from("UDP/TCP Sockets"),
            String::from("C# Harmony Lib"),
            String::from("IL Code"),
            String::from("Unity Game Engine"),
            String::from("Python/Django"),
            String::from("PostgreSQL"),
            String::from("Git"),
        ], 
        features: vec![
            FeatureItem { 
                title: String::from("Singleton Object Design"),
                description: String::from("Upon looking at the code of the game it's clear that it was made without multiplayer in mind. All the controller classes for the player/skateboard are singletons. My initial plan was to spawn in other players and send each person's input data over the network to recreate their movement. However, because of the singleton design, any attempts to spawn a duplicate player resulted in a fatal error and crashing of the program.\n\nThe solution I came up with was to disable all the controller scripts before duplication and send bone transform data instead of input data. This avoided any crashes as the singleton check wouldn't run on the duplicate objects as they're disabled and also got around using the input controller by just sending the player state each frame.") 
            },
            FeatureItem {
                title: String::from("High Bandwidth"),
                description: String::from("Sending full transform data as a solution to the singleton design allowed for accurate player states across the network, although it came at a significant bandwidth cost. A player in SkaterXL has 72 bones with 3 floats representing position and 4 floats for rotation, at 4 bytes per float it totals 2016 bytes per update, so in a 10 person lobby at 30 updates per second that's 590KB/s. A download rate of 590KB/s just for player animation was not acceptable, but as I can't duplicate scripts I have no way to simulate the transforms locally.\n\nThe solution I came up with was multi-layered, first I implemented standard compression on animation data sent over the network. The next steps were to send keyframes using half-precision floats which only use 2 bytes, and send non-keyframes as deltas from the previous frame at full precision as not all bones will move/rotate the zeros compress away nicely. Lastly, I convert the rotation from a quaternion to Euler angle form eliminating a float per bone. The combination of these techniques provides very accurate animation data while reducing the average bytes per update to less than 33% of uncompressed animation at approximately 687 bytes per update.")
            }
        ],
    };

    fn update_technology_string(technology_list: &Vec<String>) -> String {
        let mut new_str = String::from("");
        for (index, technology) in technology_list.iter().enumerate() {
            new_str += technology;
            if index % 3 == 2 {
                new_str += "\n";
            } else if index != technology_list.len() - 1 {
                new_str += "  ┅  ";
            }
        }
        new_str
    }

    let tech_string = update_technology_string(&item_1.technologies);
   
    html! {
        <Container class="mb-5">
            <Row class="mb-5">
                <Column size={12} class="text-center">
                    <h2>{item_1.title}</h2>
                    <Lead>{item_1.description}</Lead>
                </Column>
            </Row>
            <Row>
                <Column size={12} lg={6} class="mb-5">
                    <img src="/static/work/technology.png" style="display: block; margin-left: auto; margin-right: auto;" height={"100px"} />
                    <h3 class="text-center">{"Technologies Used"}</h3>
                    <Container fluid={true}>
                        <Row>
                            <Column size={12} class="text-center work-tech-list">
                                <Lead>
                                    {tech_string}
                                </Lead>
                            </Column>
                        </Row>
                    </Container>
                </Column>
                <Column size={12} lg={6}>
                    <img src="/static/work/problem.png" style="display: block; margin-left: auto; margin-right: auto;" height={"100px"} />
                    <h3 class="text-center">{"Features and Development Challenges"}</h3>
                    <Accordion id="features-and-challenges">
                            {
                                item_1.features.iter().map(|feature| {
                                    html_nested! {
                                        <AccordionItem title={feature.title.to_owned()}>
                                            {feature.description.to_owned()}
                                        </AccordionItem>
                                    }
                                }).collect::<Vec<VChild<AccordionItem>>>()
                            }
                    </Accordion>
                </Column>
            </Row>
        </Container>
    }
}
