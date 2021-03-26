# lagoon

![Lagoon](./img/lagoon.png)

## Outline

Lagoon is an esoteric programming language built around the idea of `pools`,
where a `pool` is simply a set containing memory addresses, which from now on
will be referred to as `indices`. The programmer is able to toggle the presence
of said `indices` in any of the available `pools`. Each `pool` has an
associated operation that runs over each `index` when an execution is
triggered, the default of which are as follows:

- `+`: Increment the value of each `index` in the `pool`
- `-`: Decrement the value of each `index` in the `pool`
- `o`: Output the value of each `index` in the `pool`
- `i`: Set the value of each `index` in the `pool` to the user input

In order to trigger an execution, the `;` keyword (keychar?) must be used. A
`;` causes each of the `pools` to execute on all of their `indices` exactly
once. Therefore, in order to execute multiple times, multiple `;` are needed.

## Syntax

### Toggling the presence of an `index` in a `pool`

> `<index><pool>`

As explained above, this toggles the existence of `index` `<index>` in `pool`
`<pool>`. For example, to add `0` to `-`, `1` to `+` and `o`, and `3` to `i`,
one would write the following:

```lag
0- 1+ 1o 3i
```

> Note: any form of whitespace is ignored, so *prettify* as much as desired!

### Executing the current state of the `pools`

> `;`

This simply runs through each of the `pools` and maps over each of their
`indices`, applying them to their respective operations. For example, to
execute the configuration listed above, one would write:

```lag
0- 1+ 1o 3i;
```

Currently, there are no `pools` that change their contents when an execution
is triggered. This means that an execution can be stacked together to cause it
to run multiple times. i.e.:

```lag
0+;;;
```

> In this example, memory address `0` would now contain the number `3`

### Looping over sections

> `{<guard>|<closure>}`

This is where Lagoon gets more interesting. As long as every one of the
conditions in the `<guard>` is `true`, the instructions in `<closure>` will
run. A `guard` is a comma-separated list of `indices`, where each one can be
prepended by a bang, `!`. In order for a specific `index` to return `true`, it
must have a non-zero value, unless a `!` is used to negate this clause. For
example:

```lag
0+ ;;; 0+  // set `0` to `3` and then remove `0` from the `+` pool
0- 1+      // set `0` to be decremented, and `1` to be incremented
{0 | ;}    // execute the decrement and increment as long as `0` is truthy
0o 1o;     // expected output: 0 3 (will probably print ascii later)
```

Or, for a more complicated example, take multiplication:

```lag
// multipy.lag

0i 1i;        // get inputs for `0` and `1`
0i 1i

{0 |          // while `0` is non-zero
  0-;0-       // decrement `0`

  2+ 1-       // set `2` to be incremented, and `1` to be decremented
  {1 | ;}     // equivalent to transfering everying in `1` to `2`
  2+ 1-       // undo those set actions

  2- 1+ 3+    // you get this by now
  {2 | ;}     // transfer `2` back to `1`, while also incrementing `3`
  2- 1+ 3+
}

0o 1o 2o 3o;  // expected output: 0 4 0 12
```

## Installing and running

> TODO

## Credit

The idea for Lagoon came primarily from my college friend Cullen LaKemper
([@SangerC](https://github.com/SangerC)). After weeks of bugging my friends to
attempt designing an esolang, he was the first to do so.  After that we both
decided to brainstorm ideas for the language. Cullen gets all the credit for
the amazing name choice!  He and I both decided to write our implementations in
Rust, and well, this is mine.

[Here](https://github.com/SangerC/lagoonlanguage) is Cullen's implemenation in
Python. (link to Rust repo coming soon)
