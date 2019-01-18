# Welcome to Hi Wallet

This is my Hello World app. I am a new Rust developer, but I also have decades of experience as a software developer. More specifically, I have skills in UI development in a number of languages and platforms, including: IOS/Swift/Objective-C, HTML/Javascript/CSS, Android (some), and Flash/Actionscript. As a new Rust dev, I have ramped up on the language syntax and I am diving head first into the pool with this project. 

This project aims to create a crypto wallet prototype that runs as a native app on all major desktop operating systems and eventually mobile. In this experiment, we will utilize the OrbTk project from the Redox OS team, which is working on an operating system built with Rust. I researched and experimented with many of the alternatives (Yew, Azul, Conrod) and found that OrbTk fits my needs better. Less C++ and tricky dependencies like Qt and Gtk. 

As a start, this repo will focus on building on more widgets for OrbTk. 

## Roadmap

As noted, this is an experiment at first to build stuff with Rust and contribute to UI components on OrbTk. Longer term, it will be a crypto wallet for the newly-launched Grin blockchain. [Grin](https://grin-tech.org) is a mimblewimble blockchain which uses the Rust language for most of the development. I participated in the recent launch by running my own Grin server to mine blocks with both GPU and CPU miners. It was interesting but ultimately unprofitable. Nevertheless, it compounded my interest in both Rust and Grin and this is my love-letter project for both communities. 

This crypto wallet will someday:
 
* Run and connect to the local grin wallet
* Display balances and transactions
* Send grin to an address
* Set up and run a local grin node
* Support other crypto tokens


## Widgets 

The buildout of widgets is the initial task of this project. To do this, we are summarizing the widgets that have been created so far in OrbTk and comparing that with the list of standard widget components in advanced UI platforms like iOS and Flash (Builder).

Here's my local docs for reference: file:///Sandbox/rust/orbtk/target/doc/orbtk/widget/index.html

| OrbTk        | iOS         | OrbTk  |
| ------------- |:-------------:| -----:|
| Button |  UIButton  |  yes  |
| CanvasWidget |  yes  |  yes  |
| Center |  ?  |  yes  |
| CheckBox |  ?  |  yes  |
| Column |  yes  |  yes  |
| Container |  yes  |  yes  |
| Context |  yes  |  yes  |
| Cursor |  yes  |  yes  |
| FontIconBlock |  yes  |  yes  |
| ImageWidget |  UIImageView  |  yes  |
| Row |  UIStackView  |  yes  |
| ScrollViewer |  UIScrollView  |  yes  |
| SharedProperty |  yes  |  yes  |
| Spacer |  yes  |  yes  |
| Stack |  yes  |  yes  |
| Switch |  yes  |  yes  |
| Template |  yes  |  yes  |
| TextBlock |  UILabel?  |  yes  |
| TextBox |  yes  |  yes  |
| TextBoxState |  yes  |  yes  |
| ToggleButton |  yes  |  yes  |
| WaterMarkTextBlock |  yes  |  yes  |
| WidgetContainer |  yes  |  yes  |


## Developer Notes

* This project currently relies on a local checkout of OrbTk in the same parent directory as this project.
* Install rustfmt: https://github.com/rust-lang/rustfmt
  * Run it with `cargo fmt`

