use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
enum Props {
    App(AppProps),
    Button(ButtonProps),
    Panel(PanelProps),
    Text(TextProps),
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
    fn new(root: &str) -> Self {
        Context {
            path: vec![root.to_string()],
        }
    }

    fn push(&self, segment: &str) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(segment.to_string());
        Context { path: new_path }
    }

    fn from_path(path: &str) -> Self {
        Context {
            path: path.split('.').map(|s| s.to_string()).collect(),
        }
    }

    fn get_path(&self) -> String {
        if self.path.is_empty() {
            return "__root__".to_string();
        }
        self.path.join(".")
    }
}

fn create_element(key: &str, props: Props) -> Element {
    Element {
        key: key.to_string(),
        props,
    }
}

//text
#[derive(Debug, Serialize, Clone)]
struct TextProps {
    text: String,
}
fn Text(ctx: Context, props: TextProps) -> Vec<Element> {
    vec![Element::text(&props.text)]
}

//button
#[derive(Debug, Serialize, Clone)]
struct ButtonProps {
    title: String,
}
fn Button(ctx: Context, props: ButtonProps) -> Vec<Element> {
    vec![create_element(
        ctx.push("text").get_path().as_str(),
        Props::Text(TextProps {
            text: props.title.clone(),
        }),
    )]
}

//panel
#[derive(Debug, Serialize, Clone)]
struct PanelProps {
    tabs: Vec<String>,
}
fn Panel(ctx: Context, props: PanelProps) -> Vec<Element> {
    vec![
        create_element(
            ctx.push("button1").get_path().as_str(),
            Props::Button(ButtonProps {
                title: "Hello".to_string(),
            }),
        ),
        create_element(
            ctx.push("button2").get_path().as_str(),
            Props::Button(ButtonProps {
                title: "World".to_string(),
            }),
        ),
        Element::text(props.tabs.join(",").as_str()),
    ]
}

//app
#[derive(Debug, Serialize, Clone)]
struct AppProps;
fn App(ctx: Context, props: AppProps) -> Vec<Element> {
    vec![
        create_element(
            ctx.push("panel1").get_path().as_str(),
            Props::Panel(PanelProps {
                tabs: vec!["Hello".to_string(), "World".to_string()],
            }),
        ),
        create_element(
            ctx.push("panel2").get_path().as_str(),
            Props::Panel(PanelProps {
                tabs: vec!["Heehe".to_string(), "Haha".to_string()],
            }),
        ),
    ]
}

fn render(ctx: Context, element: Element) -> Vec<Element> {
    let children = match &element.props {
        Props::App(props) => App(ctx.clone(), props.clone()),
        Props::Button(props) => Button(ctx.clone(), props.clone()),
        Props::Panel(props) => Panel(ctx.clone(), props.clone()),
        Props::Text(props) => Text(ctx.clone(), props.clone()),
        Props::Last => vec![],
    };

    let mut result = vec![element];
    for child in children {
        result.extend(render(Context::from_path(&child.key), child));
    }
    result
}

//foo => [a,b]

fn main() {
    let ctx = Context::new("app");
    let app = create_element(ctx.get_path().as_str(), Props::App(AppProps {}));
    let rendered_app = render(ctx, app);

    // Convert the rendered app to JSON and print it
    let json = serde_json::to_string_pretty(&rendered_app).unwrap();
    println!("{}", json);
}
