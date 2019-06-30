# Basil

Partial implementation of
[Basil - Material Design](https://material.io/design/material-studies/basil.html).

Features of the corresponding framework as demonstrated on https://basil.crossperf.com/:

* Critical css is calculated in `O(n)` where `n` is the number of classes used on the site.
* Css selectors are mapped to class names that are applied directly to each matching element.
* Data is transferred via WebSockets when switching pages if JavaScript is enabled.
* Works without JavaScript.
* Images are lazy-loaded.
* The same code is used on the client-side as on the server-side.
