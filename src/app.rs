use yew::prelude::*;

use yew_router::service::RouteService;
use yew_router::Switch;

use crate::widgets::tabs::Tabs;

use crate::app::pages::blog::BlogRoute;

mod pages;
mod test_pages;

const TAB_LIST: &[&str] = &["Home", "Blog", "About", "Lorem"];

pub struct Model {
	route: Option<AppRoute>,
	route_service: RouteService<()>,
	link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
	RouteChanged,
	ChangeRoute(String),
	TabClicked(usize),
	BlogRoute(String),
}

#[derive(Switch, Clone)]
pub enum AppRoute {
	#[to = "/test!"]
	Test,
	#[to = "/!"]
	Root,
	#[to = "/home!"]
	Home,
	#[to = "/blog{*}"]
	Blog(BlogRoute),
	#[to = "/about!"]
	About,
	#[to = "/lorem!"]
	Lorem,
}

impl Model {
	fn render(&self) -> Html {
		let starting_tab = match &self.route {
			None => None,
			Some(AppRoute::Test) => None,
			Some(AppRoute::Root) => Some(0),
			Some(AppRoute::Home) => Some(0),
			Some(AppRoute::Blog(_)) => Some(1),
			Some(AppRoute::About) => Some(2),
			Some(AppRoute::Lorem) => Some(3),
		};

		html! {
			<div class="container">
				<div class="header">
					<Tabs tabs=TAB_LIST start_selected=starting_tab callback=self.link.callback(Msg::TabClicked)/>
				</div>
				{
					// Render page based off path
					match &self.route {
						Some(AppRoute::Test) => html!{<test_pages::fetch::Model/>},
						None => self.render_notfound(),
						Some(AppRoute::Root) => self.render_root(),
						Some(AppRoute::Home) => html!{<pages::homepage::Model/>},
						Some(AppRoute::Blog(route)) => html!{<pages::blog::Model route=route callback=self.link.callback(Msg::BlogRoute)/>},
						Some(AppRoute::About) => html!{<pages::about::Model/>},
						Some(AppRoute::Lorem) => html!{<test_pages::lorem::Model/>},
					}
				}
				<div class="footer">
				</div>
			</div>
		}
	}

	fn render_notfound(&self) -> Html {
		html! {<pages::notfound::Model/>}
	}

	fn render_root(&self) -> Html {
		html! {<pages::homepage::Model/>}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		// Set up routing
		let mut route_service: RouteService<()> = RouteService::new();
		let callback = link.callback(|_| Msg::RouteChanged);
		route_service.register_callback(callback);

		let route = AppRoute::switch(route_service.get_route());

		Model {
			route,
			route_service,
			link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::RouteChanged => {
				self.route = AppRoute::switch(self.route_service.get_route());
			}
			Msg::ChangeRoute(route_string) => {
				self.route_service.set_route(&route_string, ());
				self.update(Msg::RouteChanged);
			}
			Msg::TabClicked(tab) => {
				let route_string = format!("/{}", TAB_LIST[tab]);
				self.update(Msg::ChangeRoute(route_string));
			}
			Msg::BlogRoute(route_string) => {
				self.update(Msg::ChangeRoute(route_string));
			}
		}
		true
	}

	fn view(&self) -> Html { self.render() }
}
