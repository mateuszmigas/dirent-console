use serde::Serialize;

#[derive(Debug, Serialize)]
enum Props {
    App(AppProps),
    Button(ButtonProps),
    Box(PanelProps),
    Panel(PanelProps),
    Div,
    Empty,
}

#[derive(Debug, Serialize)]
struct Element {
    key: String,
    props: Props,
    children: Vec<Element>,
}

impl Element {
    fn text(text: &str) -> Self {
        Element {
            key: "__text__".to_string(),
            props: Props::Empty,
            children: vec![],
        }
    }

    fn new(children: Vec<Element>) -> Self {
        Element {
            key: "__container__".to_string(),
            props: Props::Empty,
            children: children,
        }
    }
}

//context
#[derive(Clone, Copy)]
struct Context {}
impl Context {
    fn create_element(&self, key: &str, props: Props, children: Vec<Element>) -> Element {
        Element {
            key: key.to_string(),
            props,
            children,
        }
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
        "panel",
        Props::Div,
        vec![
            ctx.create_element(
                "button1",
                Props::Button(ButtonProps {
                    title: "Hello".to_string(),
                }),
                vec![],
            ),
            ctx.create_element(
                "button2",
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
        "app2",
        Props::Empty,
        vec![
            ctx.create_element(
                "panel1",
                Props::Panel(PanelProps {
                    tabs: vec!["Hello".to_string(), "World".to_string()],
                }),
                vec![],
            ),
            ctx.create_element(
                "panel2",
                Props::Panel(PanelProps {
                    tabs: vec!["Heehe".to_string(), "Haha".to_string()],
                }),
                vec![],
            ),
        ],
    )
}

fn render(ctx: &Context, element: Element) -> Element {
    // Match on the props to determine which component to call
    let rendered_element = match element.props {
        Props::App(props) => App(ctx, props),
        Props::Button(props) => Button(ctx, props),
        Props::Panel(props) => Panel(ctx, props),
        Props::Box(props) => Panel(ctx, props), // Box uses Panel implementation
        Props::Div | Props::Empty => Element::new(element.children),
    };

    // Recursively render all children
    Element {
        key: element.key,
        props: rendered_element.props,
        children: rendered_element
            .children
            .into_iter()
            .map(|child| render(ctx, child))
            .collect(),
    }
}

fn main() {
    let ctx = Context {};
    // Create the initial app element
    let app = ctx.create_element("app", Props::App(AppProps {}), vec![]);

    // Render the entire tree
    let rendered_app = render(&ctx, app);

    // Convert the rendered app to JSON and print it
    let json = serde_json::to_string_pretty(&rendered_app).unwrap();
    println!("{}", json);
}
