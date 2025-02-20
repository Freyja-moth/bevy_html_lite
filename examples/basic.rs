use bevy::prelude::*;
use bevy_html_lite::prelude::*;

fn tester(_: Trigger<Pointer<Click>>) {}

fn main() {
    let _s = sections!(<b click = tester> { "Hello there" } </b>);
}
