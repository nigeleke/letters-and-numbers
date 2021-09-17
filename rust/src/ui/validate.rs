use crate::ui::warning_icon::*;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ValidateProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<ValidateItem>,
}

pub struct Validate {
    props: ValidateProps,
}

impl Component for Validate {
    type Message = ();
    type Properties = ValidateProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Validate { props }
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
        html! {
          <span class="validated">
            { for self.props.children.iter() }
          </span>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ValidateItemProps {
    #[prop_or(true)]
    pub is_valid: bool,
    #[prop_or_default]
    pub children: Children,
}

pub struct ValidateItem {
    props: ValidateItemProps,
}

impl Component for ValidateItem {
    type Message = ();
    type Properties = ValidateItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ValidateItem { props }
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
        html! {
          <span class="validated-item">
            { for self.props.children.iter() }
            <WarningIcon is_valid=self.props.is_valid />
          </span>
        }
    }
}
