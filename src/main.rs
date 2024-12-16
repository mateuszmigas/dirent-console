use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
enum Props {
    App(AppProps),
    Button(ButtonProps),
    Box(PanelProps),
    Panel(PanelProps),
    Last,
}

#[derive(Debug, Serialize, Clone)]
struct Element {
    key: String,
    props: Props,
}

impl Element {
    fn text(text: &str) -> Self {
        Element {
            key: "__text__".to_string(),
            props: Props::Last,
        }
    }
}

//context
#[derive(Clone, Debug)]
struct Context {
    path: Vec<String>,
}

impl Context {
    fn new() -> Self {
        Context { path: Vec::new() }
    }

    fn push(&self, segment: &str) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(segment.to_string());
        Context { path: new_path }
    }

    fn get_path(&self) -> String {
        if self.path.is_empty() {
            return "__root__".to_string();
        }
        self.path.join(".")
    }
}

fn create_element(ctx: Context, props: Props) -> Element {
    Element {
        key: ctx.get_path(),
        props,
    }
}

//button
#[derive(Debug, Serialize, Clone)]
struct ButtonProps {
    title: String,
}
fn Button(ctx: Context, props: ButtonProps) -> Vec<Element> {
    vec![Element::text(&props.title)]
}

//panel
#[derive(Debug, Serialize, Clone)]
struct PanelProps {
    tabs: Vec<String>,
}
fn Panel(ctx: Context, props: PanelProps) -> Vec<Element> {
    vec![
        create_element(
            ctx.push("button1"),
            Props::Button(ButtonProps {
                title: "Hello".to_string(),
            }),
        ),
        create_element(
            ctx.push("button2"),
            Props::Button(ButtonProps {
                title: "World".to_string(),
            }),
        ),
    ]
}

//app
#[derive(Debug, Serialize, Clone)]
struct AppProps;
fn App(ctx: Context, props: AppProps) -> Vec<Element> {
    vec![
        create_element(
            ctx.push("panel1"),
            Props::Panel(PanelProps {
                tabs: vec!["Hello".to_string(), "World".to_string()],
            }),
        ),
        create_element(
            ctx.push("panel2"),
            Props::Panel(PanelProps {
                tabs: vec!["Heehe".to_string(), "Haha".to_string()],
            }),
        ),
    ]
}

fn render(ctx: Context, element: Element) -> Vec<Element> {
    let current_ctx = ctx.push(&element.key);

    let children = match &element.props {
        Props::App(props) => App(current_ctx.clone(), props.clone()),
        Props::Button(props) => Button(current_ctx.clone(), props.clone()),
        Props::Panel(props) => Panel(current_ctx.clone(), props.clone()),
        Props::Box(props) => Panel(current_ctx.clone(), props.clone()),
        Props::Last => vec![],
    };

    let mut result = vec![element];
    for child in children {
        result.extend(render(current_ctx.clone(), child));
    }
    result
}

fn main() {
    let ctx = Context::new();
    let app = create_element(ctx.push("app"), Props::App(AppProps {}));
    let rendered_app = render(ctx, app);

    // Convert the rendered app to JSON and print it
    let json = serde_json::to_string_pretty(&rendered_app).unwrap();
    println!("{}", json);
}
