use yew::prelude::*;

pub struct Model {}

pub enum Msg {}

impl Model {
	fn render(&self) -> Html {
		html! {
			<>
				<div class="left-bar">
				</div>
				<div class="main">
					<div>
					{ "About" }
					</div>
					<div>
						<img src="/static/image.jpg"/>
					</div>
				</div>
				<div class="right-bar">
				</div>
			</>
		}
	}
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Model {} }

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			_ => (),
		};
		true
	}

	fn view(&self) -> Html { self.render() }
}
