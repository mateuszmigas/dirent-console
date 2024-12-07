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

fn button(context: &Context, key: &str, text: &str) -> Node {
    context.render(|| vec![])
}

macro_rules! view {
    ($context:expr, $($item:expr),*) => {
        $context.render(|| vec![$($context.render(|| vec![$item])),*])
    };
}

fn panel(context: &Context, key: &str, title: &str, include_button: bool) -> Node {
    view!(
        context,
        button(context, "button1", title),
        if include_button {
            button(context, "button2", "Input")
        } else {
            Node::empty()
        },
        button(context, "button1", title)
    )
}

fn app(context: &Context) -> Node {
    view!(
        context,
        panel(context, "panel1", "Left", true),
        panel(context, "panel2", "Right", false)
    )
}
