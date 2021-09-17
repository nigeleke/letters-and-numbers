use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct WarningIconProps {
    #[prop_or_default]
    pub is_valid: bool,
}

pub struct WarningIcon {
    props: WarningIconProps,
}

impl Component for WarningIcon {
    type Message = ();
    type Properties = WarningIconProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        WarningIcon { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props != props;
        self.props = props;
        changed
    }

    fn view(&self) -> Html {
        if self.props.is_valid {
            html! {
              <> </>
            }
        } else {
            html! {
              <i
                class=classes!("warning-icon", "fa", "fa-warning", "w3-small", "w3-text-theme") />
            }
        }
    }
}
