# bevy-chess
A basic 3d chess game using [bevy](https://bevyengine.org/) based on [this tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/)

This project largely follows the tutorial, but since it's using a relatively old version of Bevy (0.4) and I followed
the tutorial (rather than going straight to the finished code) this isn't actually a fork for the [original project](https://github.com/guimcaballero/bevy_chess)

In addition to changes required for compatability with Bevy, I've also used the 0.12 version of [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking)
which has changed how some of the selection/highlighting code works (basically letting bevy_mod_picking handle that instead)

There are also some smaller stylistic changes to try different things and suit my own preferences - the biggest change
is probably in pieces.rs where I've tried to reduce duplication and take advantage of enums and pattern matching
