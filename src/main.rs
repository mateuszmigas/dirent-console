fn main() {
    let context = Context::new(1, "dark");
    let root = app(&context);
    println!("{:?}", root);
}

#[derive(Debug, Clone)]
struct Node {
    key: String,
    children: Vec<Node>,
}

impl Node {
    fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }
}

struct Context {
    user_id: u32,
    theme: String,
}

macro_rules! view {
    ($context:expr, $(<$tag:ident key=$key:expr, props=[$($prop:expr),*] />),*) => {
        $context.render(|| vec![
            $(
                {
                    let mut element = $tag($($prop),*);
                    element = element.key($key);
                    element
                }
            ),*
        ])
    };
}

impl Context {
    fn new(user_id: u32, theme: &str) -> Self {
        Self {
            user_id,
            theme: theme.to_string(),
        }
    }

    fn render(&self, nodes: impl Fn() -> Vec<Node>) -> Node {
        Node::new().add_children(nodes())
    }
}

impl Node {
    fn empty() -> Self {
        Self::new()
    }

    fn new() -> Self {
        Self {
            key: "".to_string(),
            children: vec![],
        }
    }

    fn add_children(mut self, children: Vec<Node>) -> Self {
        self.children.extend(children);
        self
    }
}

fn button(context: &Context, text: &str) -> Node {
    context.render(|| vec![])
}

fn each(items: Vec<Node>) -> Node {
    Node::empty().add_children(items)
}

fn show(condition: bool, node: Node) -> Node {
    if condition {
        node
    } else {
        Node::empty()
    }
}

fn panel(context: &Context, title: &str, include_button: bool) -> Node {
    view!(
        context,
        <button key="button1", props=[context, title] />,
        <show key="show1", props=[include_button, button(context, "Input")] />,
        <button key="button2", props=[context, title] />,
        <each key="each1", props=[
            ["1", "2", "3"]
                .iter()
                .map(|b| button(context, b).key(b))
                .collect()
        ] />
    )
}

fn app(context: &Context) -> Node {
    view!(
        context,
        <panel key="panel1", props=[context, "Left", true] />,
        <panel key="panel2", props=[context, "Right", false] />
    )
}
