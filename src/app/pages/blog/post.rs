use anyhow::Result;

use yew::prelude::*;

use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::vnode::VNode;

use pulldown_cmark::{html, Parser};

use web_sys::Node;

pub struct Model {
	link: ComponentLink<Self>,
	_fetch_handle: FetchTask,
	post: String,
	content: Option<String>,
	callback: Callback<String>,
}

pub enum Msg {
	Error,
	BlogRoute(String),
	GotPost(String),
}

#[derive(Clone, Properties)]
pub struct Props {
	pub post: String,
	pub callback: Callback<String>,
}

fn html_to_node(body: &str) -> Html {
	let div = web_sys::window()
		.unwrap()
		.document()
		.unwrap()
		.create_element("div")
		.unwrap();
	div.set_inner_html(body);
	let node = Node::from(div);
	VNode::VRef(node)
}

fn markdown_to_html(body: &str) -> String {
	let parser = Parser::new(body);
	let mut html_text = String::new();
	html::push_html(&mut html_text, parser);
	html_text
}

impl Model {
	fn render(&self) -> Html {
		html! {
			<>
				<div class="left-bar">
				</div>
				<div class="main">
				{
					if let Some(content) = &self.content {
						html_to_node(&markdown_to_html(content))
					}
					else {
						html! { "Loading post..." }
					}
				}
				</div>
				<div class="right-bar">
				</div>
			</>
		}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let callback = link.callback(|response: Response<Result<String>>| {
			if response.status().is_success() {
				Msg::GotPost(response.into_body().unwrap())
			}
			else {
				Msg::Error
			}
		});

		let path = format!("/static/{}.md", props.post);
		let get_request = Request::get(&path).body(Nothing).unwrap();
		let handle = FetchService::new().fetch(get_request, callback).unwrap();

		Model {
			link,
			_fetch_handle: handle,
			post: props.post,
			content: None,
			callback: props.callback,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Error => (),
			Msg::GotPost(content) => {
				self.content = Some(content);
			}
			Msg::BlogRoute(route_string) => {
				self.callback.emit(route_string);
			}
		};
		true
	}

	fn view(&self) -> Html { self.render() }
}
