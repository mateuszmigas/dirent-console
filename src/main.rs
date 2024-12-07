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

macro_rules! view {
    ($context:expr, $($item:expr),*) => {
        $context.render(|| vec![$($context.render(|| vec![$item])),*])
    };
}

fn panel(context: &Context, title: &str, include_button: bool) -> Node {
    view!(
        context,
        button(context, title).key("button1"),
        if include_button {
            button(context, "Input")
        } else {
            Node::empty()
        },
        button(context, title)
    )
}

fn app(context: &Context) -> Node {
    view!(
        context,
        panel(context, "Left", true).key("panel1"),
        panel(context, "Right", false).key("panel2")
    )
}
