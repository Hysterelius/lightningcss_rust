use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::Browsers,
};

const STYLESHEET: &str = r#"
@charset "UTF-8";
@media screen and (max-width: 30em) {
  .flex-20-s {
    flex: 0 0 20%; } }

@media screen and (min-width: 30em) and (max-width: 60em) {
  .max-width-half-m {
    max-width: 50%; } }

@media screen and (min-width: 60em) {
  .max-width-quarter-l {
    max-width: 25%; }
  .mw-50-l {
    max-width: 50%; }
  .mw-33-l {
    max-width: 33%; }
  .mw-67-l {
    max-width: 67%; } }

html {
  font-size: 62.5%; }

@media screen and (min-width: 30em) {
  html {
    font-size: 75%; } }

body {
  font-family: "Fira Sans", Helvetica, Arial, sans-serif;
  background-color: white;
  font-size: 1.5rem;
  line-height: 1.6;
  /* Ensure the footer is always at the bottom of the screen */
  min-height: 100vh;
  display: flex;
  flex-direction: column; }
  body > main {
    flex: 1; }

code {
  padding: 0.2rem 0.5rem;
  margin: 0 0.2rem;
  font-size: 90%;
  white-space: nowrap;
  border: 1px solid #e1e1e1;
  border-radius: 4px;
  overflow: auto; }

pre > code {
  display: block;
  white-space: pre;
  width: 100%; }

@media (min-width: 64rem) {
  .buttons {
    min-height: 70px; } }

@media (max-width: 64rem) {
  .buttons {
    min-height: 0px; } }

.button {
  display: inline-block;
  width: 100%;
  min-height: 38px;
  padding: 8px 30px;
  text-align: center;
  font-weight: 600;
  letter-spacing: 0.1rem;
  text-transform: uppercase;
  text-decoration: none;
  border-radius: 4px;
  border: 1px solid;
  cursor: pointer;
  box-sizing: border-box;
  white-space: nowrap; }
  .button:hover, .button:focus {
    outline: 0; }

p {
  margin-top: 0;
  margin-bottom: 2.5rem; }

.button.button-secondary {
  font-size: 0.8em; }

.button-additional {
  display: inline-block;
  text-align: center;
  width: 100%;
  font-size: 0.8em; }

code.code-header {
  background-color: rgba(127, 127, 127, 0.25);
  font-size: 2rem;
  display: inline-block;
  margin: 0 0 3px 0; }

code.copyable {
  user-select: all; }

header h1,
section h2 {
  z-index: 999999;
  position: relative;
  letter-spacing: 1px;
  font-weight: 300; }

header h1 {
  font-family: "Alfa Slab One", serif;
  font-size: 8rem;
  margin-bottom: 0;
  margin-top: 0;
  line-height: 1.2;
  font-weight: 300;
  letter-spacing: 1px; }

header h2 {
  font-size: 4.2rem;
  line-height: 1.25;
  font-weight: 300; }

section h3 {
  margin-top: 0;
  line-height: 1.3; }

section p,
code {
  line-height: 1.6; }

header .button.button-download {
  color: black;
  background-color: #ffc832;
  border-color: #ffc832;
  width: 100%;
  padding: 20px;
  height: auto;
  font-size: 2.25rem;
  margin-top: 20px; }
  header .button.button-download:hover, header .button.button-download:focus {
    border-color: black; }

h2.subtitle {
  margin: 0;
  font-family: "Fira Sans", Helvetica, Arial, sans-serif;
  font-size: 3rem;
  font-weight: 600;
  color: #2a3439; }

section {
  padding: 30px 0 60px 0; }
  section header {
    padding: 10px 0 30px 0;
    display: inline-block; }
    section header h2 {
      margin: 0;
      padding: 0;
      letter-spacing: 1px;
      font-size: 3rem;
      line-height: 1.25; }
  section h2 {
    font-family: "Fira Sans", Helvetica, Arial, sans-serif;
    font-weight: 800; }
  section h3 {
    font-weight: 600;
    line-height: 1.3; }
  section p {
    margin-top: 0;
    margin-bottom: 30px; }

"#;

fn main() {
    let mut stylesheet = StyleSheet::parse(STYLESHEET, ParserOptions::default()).unwrap();

    stylesheet
        .minify(MinifyOptions::default())
        .expect("Failed to minify css");

    let res = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .expect("Failed to write css to file");

    println!("{}", res.code);
}
