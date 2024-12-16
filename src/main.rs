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
fn Text(props: TextProps) -> Vec<Element> {
    vec![Element::text(&props.text)]
}

//button
#[derive(Debug, Serialize, Clone)]
struct ButtonProps {
    title: String,
}
fn Button(props: ButtonProps) -> Vec<Element> {
    vec![Element::text(&props.title)]
}

//panel
#[derive(Debug, Serialize, Clone)]
struct PanelProps {
    tabs: Vec<String>,
}
fn Panel(props: PanelProps) -> Vec<Element> {
    vec![
        create_element(
            "button1",
            Props::Button(ButtonProps {
                title: "Hello".to_string(),
            }),
        ),
        create_element(
            "button2",
            Props::Button(ButtonProps {
                title: "World".to_string(),
            }),
        ),
    ]
}

//app
#[derive(Debug, Serialize, Clone)]
struct AppProps;
fn App(props: AppProps) -> Vec<Element> {
    vec![
        create_element(
            "panel1",
            Props::Panel(PanelProps {
                tabs: vec!["Hello".to_string(), "World".to_string()],
            }),
        ),
        create_element(
            "panel2",
            Props::Panel(PanelProps {
                tabs: vec!["Heehe".to_string(), "Haha".to_string()],
            }),
        ),
    ]
}

fn render(key: &str, element: Element) -> Vec<Element> {
    let element_with_path = Element {
        key: key.to_string(),
        props: element.props.clone(),
    };

    let children = match element.props {
        Props::App(props) => App(props),
        Props::Button(props) => Button(props),
        Props::Panel(props) => Panel(props),
        Props::Text(props) => Text(props),
        Props::Last => vec![],
    };

    let mut result = vec![element_with_path];
    for child in children {
        let child_path = format!("{}.{}", key, child.key);
        result.extend(render(&child_path, child));
    }
    result
}

//foo => [a,b]

fn main() {
    let app = create_element("root", Props::App(AppProps {}));
    let rendered_app = render("__root__", app);
    let json = serde_json::to_string_pretty(&rendered_app).unwrap();
    println!("{}", json);
}
