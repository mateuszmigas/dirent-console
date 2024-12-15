use serde::Serialize;

#[derive(Debug, Serialize)]
enum Props {
    Button(ButtonProps),
    Box(PanelProps),
    Panel(PanelProps),
    Div,
    Empty,
}

#[derive(Debug, Serialize)]
struct Element {
    props: Props,
    children: Vec<Element>,
}

impl Element {
    fn text(text: &str) -> Self {
        Element {
            props: Props::Empty,
            children: vec![],
        }
    }

    fn new(children: Vec<Element>) -> Self {
        Element {
            props: Props::Empty,
            children: children,
        }
    }
}

//context
#[derive(Clone, Copy)]
struct Context {}
impl Context {
    fn create_element(&self, props: Props, children: Vec<Element>) -> Element {
        Element { props, children }
    }
}

//button
#[derive(Debug, Serialize)]
struct ButtonProps {
    title: String,
}
fn Button(ctx: &Context, props: ButtonProps) -> Element {
    Element::text(&props.title)
}

//box
#[derive(Debug, Serialize)]
struct PanelProps {
    tabs: Vec<String>,
}
fn Panel(ctx: &Context, props: PanelProps) -> Element {
    ctx.create_element(
        Props::Div,
        vec![
            ctx.create_element(
                Props::Button(ButtonProps {
                    title: "Hello".to_string(),
                }),
                vec![],
            ),
            ctx.create_element(
                Props::Button(ButtonProps {
                    title: "World".to_string(),
                }),
                vec![],
            ),
        ],
    )
}

//app
#[derive(Debug, Serialize)]
struct AppProps;
fn App(ctx: &Context, props: AppProps) -> Element {
    ctx.create_element(
        Props::Empty,
        vec![
            ctx.create_element(
                Props::Panel(PanelProps {
                    tabs: vec!["Hello".to_string(), "World".to_string()],
                }),
                vec![],
            ),
            ctx.create_element(
                Props::Panel(PanelProps {
                    tabs: vec!["Heehe".to_string(), "Haha".to_string()],
                }),
                vec![],
            ),
        ],
    )
}

fn main() {
    let ctx = Context {};
    let props = AppProps {};
    let app = App(&ctx, props);

    // Convert the app to JSON and print it
    let json = serde_json::to_string_pretty(&app).unwrap();
    println!("{}", json);
}
