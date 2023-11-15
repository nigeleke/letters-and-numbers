# Letters and Numbers

## Introduction

This repository started life as sample programs for a presentation on Prolog, leading up to an implementation of Letters and Numbers.

It has since become a breeding ground for other Letters and Numbers examples.

## Prolog

Demo Prolog programs for a presentation, based on an article published in Overload for the [ACCU](https://members.accu.org/index.php) and
presented to the [Ulladulla Software Developers](https://nigel-eke.com/ulladulla-software-developers) many years later.

## Scala

I was interested to see how Scala, being an expressive and concise language, was able to handle the Letters and Numbers problem.

This was later extended as to compile to ScalaJS, and explore a ScalaJS/Laminar front-end. 

Warning - The Laminar code and constructs are not pretty - especially wrt to signal handling and reactiveness - (based on lack of experience in both ScalaJS and Laminar)

## Rust

This is my first Rust program, so don't take this as an idiomatic way to write Rust. Nevertheless I wanted a reasonable sized sample program to get my teeth stuck into.

## Docker

An simple example used for a Docker presentation. Brings together the Scala and Rust examples in a simple client/server example. The front-end uses [Rust](https://www.rust-lang.org/) / [Yew](https://yew.rs/) and the backend uses [Scala](https://scala-lang.org/) / [http4s](https://http4s.org/). 

## Zig

This is my first Zig program, so don't take this as an idiomatic way to write Zig. It was written to compare against Rust - and my noobie frustrations with Rust's borrow checker.

It may be because I don't have enough experience in Zig but in the end this isn't giving me the same safety as Rust; During development  I have had too many errors occurring at runtime.  The currently implementation is not completed. The Expression.from() method is very slow.

I haven't found enough benefits of Zig over C++ to make me want to delve deeper and I have found enough frustrations in the idioms of the language such that I'll park it and switch back to Rust.

I'm more than happy for an experienced Zigger to take this example further so I can better understand the concepts I'm clearly missing.

## V

This is my first V program, so don't take this as an idiomatic way to write V. It was written to try out the language wrt this program. It has proved quite successful, easy to learn with a simple syntax.  The UI & Web libraries may get investigated further and included here for comparison too.