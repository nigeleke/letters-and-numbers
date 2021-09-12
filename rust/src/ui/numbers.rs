use crate::ui::number::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NumbersProps {
  pub value: NumbersValue,
  pub on_update: Callback<NumbersValue>,
  pub active: bool,
}

pub type NumbersValue = Option<(i32, i32, i32, i32, i32, i32)>;

pub enum NumbersMsg {
  ValidateNumber(usize, Option<i32>),
}

pub struct Numbers {
  props: NumbersProps,
  link: ComponentLink<Self>,
  numbers: Vec<Option<i32>>,
}

impl Component for Numbers {
  type Message = NumbersMsg;
  type Properties = NumbersProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Numbers {
      props,
      link,
      numbers: vec![None; 6],
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      NumbersMsg::ValidateNumber(id, maybe_number) => {
        self.numbers[id] = maybe_number;
        let valid_numbers = self.numbers.iter().flat_map(|n| *n).collect::<Vec<i32>>();
        let maybe_numbers = Some(valid_numbers)
          .filter(|ns| ns.len() == 6)
          .map(|ns| (ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]));
        self.props.on_update.emit(maybe_numbers);
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    let changed = self.props != props;
    log::info!("Numbers::change from {:?} to {:?}", self.props, props);
    self.props = props;
    changed
  }

  fn view(&self) -> Html {
    let cb = |id| move |update| NumbersMsg::ValidateNumber(id, update);
    html! {
      <div class=classes!("w3-center", "w3-section", "w3-auto", "w3-row", "w3-row-padding")>
        <Number on_update=self.link.callback(cb(0)) active=self.props.active />
        <Number on_update=self.link.callback(cb(1)) active=self.props.active />
        <Number on_update=self.link.callback(cb(2)) active=self.props.active />
        <Number on_update=self.link.callback(cb(3)) active=self.props.active />
        <Number on_update=self.link.callback(cb(4)) active=self.props.active />
        <Number on_update=self.link.callback(cb(5)) active=self.props.active />
      </div>
    }
  }
}
