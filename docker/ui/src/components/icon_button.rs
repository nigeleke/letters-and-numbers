// use crate::frontend::actions::action::*;
//
// use yew::prelude::*;
//
// pub struct IconButton {
//   props: Props,
//   link: ComponentLink<Self>,
// }
//
// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//   pub action: Action,
// }
//
// #[derive(Debug)]
// pub enum Msg {
//   Execute(MouseEvent),
// }
//
// impl Component for IconButton {
//   type Message = Msg;
//   type Properties = Props;
//
//   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//     Self { props, link }
//   }
//
//   fn update(&mut self, msg: Self::Message) -> ShouldRender {
//     match msg {
//       Msg::Execute(_) => {
//         self.props.action.on_execute.emit(());
//         false
//       }
//     }
//   }
//
//   fn change(&mut self, props: Self::Properties) -> ShouldRender {
//     let render = self.props != props;
//     self.props = props;
//     render
//   }
//
//   fn view(&self) -> Html {
//     let onclick = self.link.callback(Msg::Execute);
//     html! {
//       <button
//         class="icon-button"
//         type="button"
//         disabled=!self.props.action.enabled
//         hidden=!self.props.action.visible
//         onclick=onclick>
//         <span class=self.props.action.icon_class.to_string() />
//       </button>
//     }
//   }
// }
