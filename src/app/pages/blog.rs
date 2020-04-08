use yew::prelude::*;

use yew_router::Switch;

mod page;
mod post;

pub struct Model {
	link: ComponentLink<Self>,
	route: BlogRoute,
	callback: Callback<String>,
}

pub enum Msg {
	BlogRoute(String),
}

#[derive(Clone, Properties)]
pub struct Props {
	pub route: BlogRoute,
	pub callback: Callback<String>,
}

#[derive(Switch, Clone)]
pub enum BlogRoute {
	#[to = "!"]
	Root,
	#[to = "/page/{}"]
	Page(u16),
	#[to = "/post/{*}"]
	Post(String),
}

impl Model {
	fn render(&self) -> Html {
		match &self.route {
			BlogRoute::Root => self.render_root(),
			BlogRoute::Page(page) => {
				html! {<page::Model page=page callback=self.link.callback(Msg::BlogRoute)/>}
			}
			BlogRoute::Post(post) => {
				html! {<post::Model post=post callback=self.link.callback(Msg::BlogRoute)/>}
			}
		}
	}

	fn render_root(&self) -> Html {
		html! {<page::Model page=0 callback=self.link.callback(Msg::BlogRoute)/>}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Model {
			link,
			route: props.route,
			callback: props.callback,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::BlogRoute(route_string) => {
				self.callback.emit(route_string);
			}
		};
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.route = props.route;
		true
	}

	fn view(&self) -> Html { self.render() }
}
