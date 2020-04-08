use anyhow::Result;

use yew::prelude::*;

use yew::format::Nothing;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use serde::{Deserialize, Serialize};

use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct Blog {
	posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
struct Post {
	name: String,
}

pub struct Model {
	link: ComponentLink<Self>,
	_fetch_handle: FetchTask,
	page: u16,
	content: Option<Blog>,
	callback: Callback<String>,
}

pub enum Msg {
	Error,
	BlogRoute(String),
	GotBlog(String),
}

#[derive(Clone, Properties)]
pub struct Props {
	pub page: u16,
	pub callback: Callback<String>,
}

impl Model {
	fn render(&self) -> Html {
		html! {
			<>
				<div class="left-bar">
				</div>
				<div class="main">
				{
					if let Some(blog) = &self.content {
						blog.posts
							.iter()
							.map(|post| {
								let location = format!("/Blog/post/{}", post.name);
								html! {
									<div>
											<a onclick=self.link.callback(move |_| Msg::BlogRoute(location.clone()))>
											<u>
												{ &post.name }
											</u>
											</a>
									</div>
								}
							})
							.collect::<Html>()
					}
					else {
						html! { "Loading page..." }
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
				Msg::GotBlog(response.into_body().unwrap())
			}
			else {
				Msg::Error
			}
		});

		let get_request = Request::get("/static/blog.json").body(Nothing).unwrap();
		let handle = FetchService::new().fetch(get_request, callback).unwrap();

		Model {
			link,
			_fetch_handle: handle,
			page: props.page,
			content: None,
			callback: props.callback,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Error => (),
			Msg::GotBlog(blog) => {
				let deserialised = from_str(&blog).unwrap();
				self.content = Some(deserialised);
			}
			Msg::BlogRoute(route_string) => {
				self.callback.emit(route_string);
			}
		};
		true
	}

	fn view(&self) -> Html { self.render() }
}
