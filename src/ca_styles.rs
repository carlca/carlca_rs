#![allow(nonstandard_style)]
use stylist::{css, StyleSource};

pub fn app_styles(font_name: &str) -> StyleSource {
  css! {
    h1, h3, p {
      font-family: ${font_name};
      color: #202060;
      margin: 10px;
    }
    ul {
      font-family: ${font_name};
      color: purple;
    }
    html, body {
      margin: 0;
      padding: 0;
      height: 100%;
      width: 100%;
    }
    body {
      background-image: url("assets/cherry_blossom2.png");
      background-size: cover;
      background-position: center;
    }
  }
}

