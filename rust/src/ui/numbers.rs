use crate::ui::number::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NumbersProps {
  pub active: bool,
  pub on_update: Callback<NumbersValue>,
}

#[derive(Debug)]
pub enum NumbersValue {
  Valid(i32, i32, i32, i32, i32, i32),
  Invalid(String, String, String, String, String, String),
}

#[derive(Debug)]
pub enum NumbersMsg {
  ValidateNumber(usize, NumberValue),
}

pub struct Numbers {
  props: NumbersProps,
  link: ComponentLink<Self>,
  values: Vec<NumberValue>,
}

impl Component for Numbers {
  type Message = NumbersMsg;
  type Properties = NumbersProps;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Numbers {
      props,
      link,
      values: vec![NumberValue::Invalid("".to_string()); 6],
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    log::info!("Numbers::update {:?}", msg);
    fn valid_set(_ns: &[i32]) -> bool {
      let lows1 = 1..=10;
      let lows2 = 1..=10;
      let highs = (25..=100).step_by(25);
      let allowed_numbers = lows1.chain(lows2).chain(highs);
      log::info!("valid_set: {:?}", allowed_numbers);
      true
    }
    match msg {
      NumbersMsg::ValidateNumber(id, number_value) => {
        self.values[id] = number_value;
        let is = self
          .values
          .iter()
          .map(|nv| match *nv {
            NumberValue::Valid(i) => i,
            NumberValue::Invalid(_) => 0,
          })
          .collect::<Vec<i32>>();
        let ss = self
          .values
          .iter()
          .map(|nv| match &*nv {
            NumberValue::Valid(i) => i.to_string(),
            NumberValue::Invalid(s) => s.to_string(),
          })
          .collect::<Vec<String>>();
        let validated_numbers = if is.len() == 6 && valid_set(&is) {
          NumbersValue::Valid(is[0], is[1], is[2], is[3], is[4], is[5])
        } else {
          NumbersValue::Invalid(
            ss[0].to_string(),
            ss[1].to_string(),
            ss[2].to_string(),
            ss[3].to_string(),
            ss[4].to_string(),
            ss[5].to_string(),
          )
        };
        self.props.on_update.emit(validated_numbers);
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    log::info!("Numbers::change from {:?} to {:?}", self.props, props);
    let changed = self.props != props;
    self.props = props;
    changed
  }

  fn view(&self) -> Html {
    let cb = |id| move |update| NumbersMsg::ValidateNumber(id, update);
    let active = self.props.active;
    html! {
      <div class=classes!("w3-center", "w3-section", "w3-auto", "w3-row", "w3-row-padding")>
        <Number on_update=self.link.callback(cb(0)) active=active />
        <Number on_update=self.link.callback(cb(1)) active=active />
        <Number on_update=self.link.callback(cb(2)) active=active />
        <Number on_update=self.link.callback(cb(3)) active=active />
        <Number on_update=self.link.callback(cb(4)) active=active />
        <Number on_update=self.link.callback(cb(5)) active=active />
      </div>
    }
  }
}
