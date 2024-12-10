fn main() {
    let context = Context::new("app", "dark");
    let nodes = app(&context);
    println!("{:?}", nodes);
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    title: String,
}

struct Context {
    theme: String,
    current_path: Vec<String>,
}

impl Context {
    fn new(key: &str, theme: &str) -> Self {
        Self {
            theme: theme.to_string(),
            current_path: vec![key.to_string()],
        }
    }

    fn with_key(&self, key: &str) -> Self {
        let mut new_path = self.current_path.clone();
        new_path.push(key.to_string());
        Self {
            theme: self.theme.clone(),
            current_path: new_path,
        }
    }
}

impl Node {
    fn new(children: Vec<Node>) -> Self {
        Self {
            children: children,
            title: "".to_string(),
        }
    }

    fn with_title(title: &str) -> Self {
        Self {
            title: title.to_string(),
            children: vec![],
        }
    }
}

fn title(_: &Context, text: &str) -> Node {
    Node::new(vec![Node::with_title(text)])
}
fn button(context: &Context, text: &str) -> Node {
    Node::new(vec![title(&context, &(text.to_string() + "title 1"))])
}

fn panel(context: &Context, title: &str) -> Node {
    Node::new(vec![Node::new(vec![
        button(&context.with_key("button1"), "Button 1"),
        button(&context.with_key("button2"), "Button 2"),
    ])])
}

fn app(context: &Context) -> Node {
    Node::new(vec![Node::new(vec![
        panel(&context.with_key("panel1"), "Left"),
        panel(&context.with_key("panel2"), "Right"),
    ])])
}
