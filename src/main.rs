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

macro_rules! view {
    ($context:expr, $({ type: $type:expr, props: [$($props:expr),*] $(, key: $key:expr)? }),* $(,)?) => {
        $context.render(|| vec![
            $(
                {
                    let mut node = $type($context, $($props),*);
                    $(node.key = $key.to_string();)?
                    node
                }
            ),*
        ])
    };
}

fn button(context: &Context, key: &str, text: &str) -> Node {
    context.render(|| vec![])
}

fn panel(context: &Context, title: &str, include_button: bool) -> Node {
    view!(
        context,
        { type: button, props: ["button1", title], key: "button1_key" },
        { type: button, props: ["button2", "Input"], key: "button2_key" },
        { type: button, props: ["button1", title] }
    )
}

fn app(context: &Context) -> Node {
    view!(
        context,
        { type: panel, props: ["Left", true], key: "panel1_key" },
        { type: panel, props: ["Right", false], key: "panel2_key" }
    )
}
