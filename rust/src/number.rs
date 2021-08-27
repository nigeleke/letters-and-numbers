pub struct Number;

impl Component for Number {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Number
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <span>Hello Component World</span>
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    // Not required - does nothing by default
  }

  fn update(&mut self, _ctx: &Context<Self>), _msg: Self::Message) -> bool {
    false
  }

}