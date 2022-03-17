// use super::validated::*;
//
// use crate::shared::number::*;
//
use yew::prelude::*;

use crate::validated::*;

//
// #[derive(Debug)]
// pub struct NumberView {
//   props: Props,
//   link: ComponentLink<Self>,
//   string_value: String,
// }
//
// #[derive(Debug)]
// pub enum Msg {
//   Updated(String),
// }
//
// #[derive(Clone, Debug, PartialEq, Properties)]
// pub struct Props {
//   pub value: Number,
//   pub on_change: Callback<Number>,
// }
//
// impl Component for NumberView {
//   type Message = Msg;
//   type Properties = Props;
//
//   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//     Self {
//       props,
//       link,
//       string_value: "".to_owned(),
//     }
//   }
//
//   fn update(&mut self, msg: Self::Message) -> ShouldRender {
//     let string_to_number = |s: &str| match s.parse::<isize>() {
//       Ok(i) => {
//         if (1..=10).contains(&i) || vec![25, 50, 75, 100].contains(&i) {
//           Number::Valid(i)
//         } else {
//           Number::Invalid
//         }
//       }
//       Err(_) => Number::Invalid,
//     };
//
//     match msg {
//       Msg::Updated(s) => {
//         let number = string_to_number(&s);
//         let notify = number != self.props.value;
//         if notify {
//           self.props.on_change.emit(number);
//           self.string_value = s;
//         };
//         false
//       }
//     }
//   }
//
//   fn change(&mut self, props: Self::Properties) -> ShouldRender {
//     let render = self.props != props;
//     if props.value == Number::Unset {
//       self.string_value = "".to_owned()
//     };
//     self.props = props;
//     render
//   }
//
//   fn view(&self) -> Html {
//     let oninput = self
//       .link
//       .callback(|event: InputData| Msg::Updated(event.value));
//     html! {
//       <div class="number">
//         <Validated valid=self.props.value.is_valid() >
//           <input
//             type="text"
//             size="2"
//             list="number-data"
//             oninput=oninput
//             autocomplete="off"
//             value=self.string_value.to_string() />
//           <datalist id="number-data">
//             <option value="1" />
//             <option value="2" />
//             <option value="3" />
//             <option value="4" />
//             <option value="5" />
//             <option value="6" />
//             <option value="7" />
//             <option value="8" />
//             <option value="9" />
//             <option value="10" />
//             <option value="25" />
//             <option value="50" />
//             <option value="75" />
//             <option value="100" />
//           </datalist>
//         </Validated>
//       </div>
//     }
//   }
// }

#[function_component(NumberView)]
pub fn number_view() -> Html {
  html! {
    <div class="number">
      <Validated valid=true use_icon=true >
        <input
          type="text"
          size="2"
          list="number-data"
          autocomplete="off" />
      </Validated>
    </div>
  }

}