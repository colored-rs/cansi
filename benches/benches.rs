#[macro_use]
extern crate criterion;

use colored::*;
use criterion::Criterion;

fn categorise_text_fn(c: &mut Criterion) {
    c.bench_function("fn categorise_text &str", |b| {
        let txt = "\u{1b}[91mHello, world!\u{1b}[0m";
        b.iter(|| cansi::categorise_text(&txt))
    });

    c.bench_function("fn categorise_text simple", |b| {
        let txt = format!("{}", "Hello, world!".bright_red());
        b.iter(|| cansi::categorise_text(&txt))
    });

    c.bench_function("fn categorise_text long no color", |b| {
        let s = cstr();
        let txt = cansi::construct_text_no_codes(&cansi::categorise_text(&s));
        b.iter(|| cansi::categorise_text(&txt))
    });

    c.bench_function("fn categorise_text long complex", |b| {
        let txt = cstr();
        b.iter(|| cansi::categorise_text(&txt))
    });

    c.bench_function("fn categorise_text long x 4 complex", |b| {
        let txt = format!("{}{}{}{}", cstr(), cstr(), cstr(), cstr());
        b.iter(|| cansi::categorise_text(&txt))
    });
}

fn construct_text_no_codes_fn(c: &mut Criterion) {
    c.bench_function("fn construct_text_no_codes", |b| {
        let s = cstr();
        let cat = cansi::categorise_text(&s);
        b.iter(|| cansi::construct_text_no_codes(&cat))
    });
}

fn cstr() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".red(),
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".blue(),
        "Donec vel metus nec nisl ultrices cursus.".green(),
        "In in enim eget felis elementum consectetur et nec nisi.".purple(),
        "Morbi vel sapien consectetur, tristique sem id, facilisis purus.".yellow(),
        "Vivamus bibendum nisi ac lacus euismod hendrerit vel ac lacus.".red(),
        " Nulla scelerisque ipsum eu lacus dignissim, a tempus arcu egestas.".white(),
        "Nulla scelerisque ipsum eu lacus dignissim, a tempus arcu egestas.".bright_red(),
        "Praesent lobortis quam sed erat egestas, et tincidunt erat rutrum.".bright_white(),
        "Nullam maximus mauris a ultricies blandit.".bright_green(),
        "Morbi eget neque eget neque viverra mollis in id lacus.".bright_purple(),
    )
}

criterion_group!(benches, categorise_text_fn, construct_text_no_codes_fn);
criterion_main!(benches);
