use yew::prelude::*;

use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::vnode::VNode;

use anyhow::Result;

use web_sys::Node;

pub struct Model {
	_link: ComponentLink<Self>,
	_fetch_handle: FetchTask,
	content: String,
}

pub enum Msg {
	DidFetch(String),
	Error,
}

fn html_to_node(body: &str) -> Html {
	let parser = pulldown_cmark::Parser::new(body);
	let mut html_text = String::new();
	pulldown_cmark::html::push_html(&mut html_text, parser);

	let div = web_sys::window()
		.unwrap()
		.document()
		.unwrap()
		.create_element("div")
		.unwrap();
	div.set_inner_html(html_text.as_str());
	let node = Node::from(div);
	VNode::VRef(node)
}

impl Model {
	fn render(&self) -> Html {
		let node = html_to_node(&self.content);

		html! {
			<div>
			{ node }
			</div>
		}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let get_request = Request::get("/static/test.md")
			.body(yew::format::Nothing)
			.unwrap();

		let callback = link.callback(|response: Response<Result<String>>| {
			if response.status().is_success() {
				Msg::DidFetch(response.into_body().unwrap())
			}
			else {
				Msg::Error
			}
		});

		let handle = FetchService::new().fetch(get_request, callback).unwrap();

		Model {
			_link: link,
			_fetch_handle: handle,
			content: String::from("Loading..."),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DidFetch(response) => self.content = response,
			Msg::Error => self.content = String::from("An error has occurred"),
		};

		true
	}

	fn view(&self) -> Html { self.render() }
}
