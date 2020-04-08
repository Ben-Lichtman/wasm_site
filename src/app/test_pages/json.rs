use yew::prelude::*;

use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use anyhow::Result;

use serde::Deserialize;

#[derive(Deserialize)]
struct Content {
	posts: Vec<Post>,
}

#[derive(Deserialize)]
struct Post {
	location: String,
}

pub struct Model {
	_link: ComponentLink<Self>,
	_fetch_handle: FetchTask,
	content: Option<Content>,
}

pub enum Msg {
	DidFetch(String),
	Error,
}

impl Model {
	fn render(&self) -> Html {
		html! {
			<>
			<div>
				{
					match &self.content {
						Some(content) => {
							html! { for content.posts.iter().map(|post| {
								html!{
									<div>
										{ &post.location }
									</div>
								}
							})}
						},
						None => html!{},
					}
				}
			</div>
			</>
		}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let get_request = Request::get("/static/blog.json")
			.body(yew::format::Nothing)
			.unwrap();

		let callback = link.callback(|response: Response<Result<_>>| {
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
			content: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::DidFetch(response) => match serde_json::from_str(&response) {
				Ok(deserialised) => self.content = deserialised,
				Err(_) => panic!("Deserialisation failed"),
			},
			Msg::Error => self.content = None,
		};

		true
	}

	fn view(&self) -> Html { self.render() }
}
