use yew::prelude::*;

pub struct Tabs {
	link: ComponentLink<Self>,
	tabs: &'static [&'static str],
	selected_tab: Option<usize>,
	callback: Callback<usize>,
}

pub enum Msg {
	Selected(usize),
}

#[derive(Clone, Properties)]
pub struct Props {
	pub tabs: &'static [&'static str],
	#[prop_or_default]
	pub start_selected: Option<usize>,
	pub callback: Callback<usize>,
}

impl Component for Tabs {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			tabs: props.tabs,
			selected_tab: props.start_selected,
			callback: props.callback,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Selected(index) => {
				self.selected_tab = Some(index);
				self.callback.emit(index);
			}
		};
		true
	}

	fn view(&self) -> Html {
		html! {
			<div class="navigation">
				{ for self.tabs.iter().enumerate().map(|(index, string)| {
					if Some(index) == self.selected_tab {
						html! {
							<a class="active" onclick=self.link.callback(move |_| Msg::Selected(index))>
								{ string }
							</a>
						}
					}
					else {
						html! {
							<a onclick=self.link.callback(move |_| Msg::Selected(index))>
								{ string }
							</a>
						}
					}
				})}
			</div>
		}
	}
}
