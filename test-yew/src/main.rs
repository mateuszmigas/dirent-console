use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

/*
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use yew::prelude::*;
#[allow(unused_parens)]
struct App {
    _marker: ::std::marker::PhantomData<()>,
    function_component: ::yew::functional::FunctionComponent<Self>,
}
impl ::yew::functional::FunctionProvider for App {
    type Properties = ();
    fn run(
        ctx: &mut ::yew::functional::HookContext,
        props: &Self::Properties,
    ) -> ::yew::html::HtmlResult {
        fn inner(_ctx: &mut ::yew::functional::HookContext, _: &()) -> Html {
            {
                let counter = ::yew::functional::Hook::run(use_state(|| 0), _ctx);
                let onclick = {
                    let counter = counter.clone();
                    move |_| {
                        let value = *counter + 1;
                        counter.set(value);
                    }
                };
                {
                    #[allow(clippy::useless_conversion)]
                    <::yew::virtual_dom::VNode as ::std::convert::From<
                        _,
                    >>::from({
                        #[allow(clippy::redundant_clone, unused_braces)]
                        let node = ::std::convert::Into::<
                            ::yew::virtual_dom::VNode,
                        >::into(
                            ::yew::virtual_dom::VTag::__new_other(
                                ::yew::virtual_dom::AttrValue::Static("div"),
                                ::std::default::Default::default(),
                                ::std::option::Option::None,
                                ::yew::virtual_dom::Attributes::Static(&[]),
                                ::yew::virtual_dom::listeners::Listeners::None,
                                ::yew::html::IntoPropValue::<
                                    ::yew::virtual_dom::VNode,
                                >::into_prop_value(
                                    ::yew::html::ChildrenRenderer::new(
                                        <[_]>::into_vec(
                                            #[rustc_box]
                                            ::alloc::boxed::Box::new([
                                                ::std::convert::Into::into({
                                                    #[allow(clippy::redundant_clone, unused_braces)]
                                                    let node = ::std::convert::Into::<
                                                        ::yew::virtual_dom::VNode,
                                                    >::into(
                                                        ::yew::virtual_dom::VTag::__new_other(
                                                            ::yew::virtual_dom::AttrValue::Static("button"),
                                                            ::std::default::Default::default(),
                                                            ::std::option::Option::None,
                                                            ::yew::virtual_dom::Attributes::Static(&[]),
                                                            ::yew::virtual_dom::listeners::Listeners::Pending(
                                                                ::std::boxed::Box::new([
                                                                    ::yew::html::onclick::Wrapper::__macro_new(onclick),
                                                                ]),
                                                            ),
                                                            ::yew::html::IntoPropValue::<
                                                                ::yew::virtual_dom::VNode,
                                                            >::into_prop_value(
                                                                ::yew::virtual_dom::VText::new(
                                                                    ::yew::virtual_dom::AttrValue::Static("+1"),
                                                                ),
                                                            ),
                                                        ),
                                                    );
                                                    node
                                                }),
                                                ::std::convert::Into::into({
                                                    #[allow(clippy::redundant_clone, unused_braces)]
                                                    let node = ::std::convert::Into::<
                                                        ::yew::virtual_dom::VNode,
                                                    >::into(
                                                        ::yew::virtual_dom::VTag::__new_other(
                                                            ::yew::virtual_dom::AttrValue::Static("p"),
                                                            ::std::default::Default::default(),
                                                            ::std::option::Option::None,
                                                            ::yew::virtual_dom::Attributes::Static(&[]),
                                                            ::yew::virtual_dom::listeners::Listeners::None,
                                                            ::yew::html::IntoPropValue::<
                                                                ::yew::virtual_dom::VNode,
                                                            >::into_prop_value(*counter),
                                                        ),
                                                    );
                                                    node
                                                }),
                                            ]),
                                        ),
                                    ),
                                ),
                            ),
                        );
                        node
                    })
                }
            }
        }
        ::yew::html::IntoHtmlResult::into_html_result(inner(ctx, props))
    }
}
#[automatically_derived]
impl ::std::fmt::Debug for App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("App<_>"))
    }
}
#[automatically_derived]
impl ::yew::html::BaseComponent for App
where
    Self: 'static,
{
    type Message = ();
    type Properties = ();
    #[inline]
    fn create(ctx: &::yew::html::Context<Self>) -> Self {
        Self {
            _marker: ::std::marker::PhantomData,
            function_component: ::yew::functional::FunctionComponent::<Self>::new(ctx),
        }
    }
    #[inline]
    fn update(
        &mut self,
        _ctx: &::yew::html::Context<Self>,
        _msg: Self::Message,
    ) -> ::std::primitive::bool {
        true
    }
    #[inline]
    fn changed(
        &mut self,
        _ctx: &::yew::html::Context<Self>,
        _old_props: &Self::Properties,
    ) -> ::std::primitive::bool {
        true
    }
    #[inline]
    fn view(&self, ctx: &::yew::html::Context<Self>) -> ::yew::html::HtmlResult {
        ::yew::functional::FunctionComponent::<
            Self,
        >::render(&self.function_component, ::yew::html::Context::<Self>::props(ctx))
    }
    #[inline]
    fn rendered(
        &mut self,
        _ctx: &::yew::html::Context<Self>,
        _first_render: ::std::primitive::bool,
    ) {
        ::yew::functional::FunctionComponent::<Self>::rendered(&self.function_component)
    }
    #[inline]
    fn destroy(&mut self, _ctx: &::yew::html::Context<Self>) {
        ::yew::functional::FunctionComponent::<Self>::destroy(&self.function_component)
    }
    #[inline]
    fn prepare_state(&self) -> ::std::option::Option<::std::string::String> {
        ::yew::functional::FunctionComponent::<
            Self,
        >::prepare_state(&self.function_component)
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
     */
