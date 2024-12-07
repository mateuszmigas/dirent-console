fn main() {
    let context = Context::new(1, "dark");
    let root = app(&context);
    println!("{:?}", root);
}

#[derive(Debug)]
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

    fn render(&self, component: impl Fn(&Self) -> Node) -> Node {
        component(self)
    }
}

impl Node {
    fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            children: vec![],
        }
    }

    fn add_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }
}

fn button(context: &Context, key: &str, text: &str) -> Node {
    Node::new(key)
}

fn panel(context: &Context, key: &str, title: &str) -> Node {
    let button1 = context.render(|_| button(context, "button1", title));

    Node::new(key)
        .add_child(button(context, "button1", title))
        .add_child(button(context, "button2", "Input"))
}

fn app(context: &Context) -> Node {
    let left_panel = panel(context, "panel1", "Left");
    let right_panel = panel(context, "panel2", "Right");

    Node::new("app")
        .add_child(left_panel)
        .add_child(right_panel)
}
