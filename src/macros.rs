#[macro_export]
macro_rules! impl_render_props {
    ($($t:ty),*) => {
        $(
            impl RenderProps for $t {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
            }
        )*
    };
}
