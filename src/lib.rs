#![allow(non_snake_case)]
mod ca_styles;
use ca_styles::app_styles;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{StyleSource, yew::Global, yew::styled_component};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct Data {
  font_name: String,
  description: String,
}

#[styled_component(App)]
pub fn app() -> Html {
  let font_name: &str = "RoughTypewriter";
  let style = app_styles(font_name);

  log!{"font_name: ", font_name};
  let data = Data {
    font_name: font_name.to_owned(),
    description: "Grungy".to_owned(),
  };
  log!(serde_json::to_string_pretty(&data).unwrap());

  let switcher: bool = false;

  let items = vec!["alpha", "beta", "gamma"];

  let style_source: StyleSource = style.into();
  html! {
    <>
      <Global css={style_source}/>
      <h1>{ "early spring" }</h1>
      <h3>{ "A creative outlet for carl caulkett" }</h3>
      if switcher {
        <p>{ "Here is a paragraph!" }</p>
      } else {
        <p>{ "Here is another paragraph!" }</p>
      }
      <p>{ "Here is a paragraph!" }</p>
      {ul(items)}  
    </>
  }
}

pub fn ul(list: Vec<&str>) -> Html {
  html! {
    <ul>
      {for list.iter().map(|item| li(item))}
    </ul>
  }
}

pub fn li(item: &str) -> Html {
  html! {<li>{item}</li>}
}