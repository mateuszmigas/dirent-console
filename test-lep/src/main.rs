use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(1);

    view! {
        <button
            on:click=move |_| set_count.set(count.get() + 1)
        >
            "Click me: "
            {count} "_" {count.get()}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}

/*

use leptos::prelude::*;
/// Props for the [`App`] component.
///
#[allow(non_snake_case)]
struct AppProps {}
#[automatically_derived]
impl AppProps {
    /**
    Create a builder for building `AppProps`.
    On the builder, call  to set the values of the fields.
    Finally, call `.build()` to create the instance of `AppProps`.
    */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> AppPropsBuilder<()> {
        AppPropsBuilder {
            fields: (),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct AppPropsBuilder<TypedBuilderFields = ()> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for AppPropsBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl AppPropsBuilder<()> {
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> AppProps {
        let () = self.fields;
        #[allow(deprecated)]
        AppProps {}.into()
    }
}
#[allow(missing_docs)]
impl ::leptos::component::Props for AppProps {
    type Builder = AppPropsBuilder;
    fn builder() -> Self::Builder {
        AppProps::builder()
    }
}
#[allow(non_snake_case, clippy::too_many_arguments)]
#[allow(clippy::needless_lifetimes)]
fn App() -> impl IntoView {
    ::leptos::prelude::untrack(move || __App())
}
#[doc(hidden)]
#[allow(
    non_snake_case,
    dead_code,
    clippy::too_many_arguments,
    clippy::needless_lifetimes
)]
pub fn __App() -> impl IntoView {
    let (count, set_count) = signal(1);
    {
        #[allow(unused_braces)]
        {
            ::leptos::prelude::View::new((
                ::leptos::tachys::html::element::button()
                    .child((
                        "Click me: ",
                        ::leptos::prelude::IntoRender::into_render({ count }),
                    ))
                    .on(::leptos::tachys::html::event::click, move |_| {
                        set_count.set(count.get() + 1)
                    }),
                ::leptos::tachys::html::element::p().child((
                    "Double count: ",
                    ::leptos::prelude::IntoRender::into_render({ move || count.get() * 2 }),
                )),
            ))
        }
    }
}
fn main() {
    leptos::mount::mount_to_body(App);
}

*/
