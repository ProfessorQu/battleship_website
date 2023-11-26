use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct OptionButtonProps {
    pub show_options: bool,
    pub option_callback: Callback<bool>,
}

#[function_component]
pub fn OptionButton(
    OptionButtonProps {
        show_options,
        option_callback
    }: &OptionButtonProps
) -> Html {
    let onclick = {
        let callback = option_callback.clone();
        let show_options = *show_options;
        Callback::from(move |_| {
            callback.emit(!show_options)
        })
    };

    let text = if *show_options {
        "Hide Options"
    } else {
        "Show Options"
    };

    html!(
        <button {onclick} class="important">{text}</button>
    )
}