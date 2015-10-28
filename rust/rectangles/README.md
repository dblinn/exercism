# Rectangles

Count the rectangles in an ASCII diagram.

Create a program to count the rectangles in an ASCII diagram like the one below.

```
   +--+
   |  |
+--+--+
|  |  |
+--+--+
```

The above diagram contains 5 rectangles:

```


+-----+
|     |
+-----+
```

```
   +--+
   |  |
   |  |
   |  |
   +--+
```

```
   +--+
   |  |
   +--+


```

```
       
       
   +--+
   |  |
   +--+
```

```
       
       
+--+
|  |
+--+
```

You may assume that the input is always a proper rectangle (i.e. the length of
every line equals the length of the first line).

## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Crates and Modules](crates-and-modules) chapter if you
haven't already, it will help you with organizing your files.

[help-page]: http://help.exercism.io/getting-started-with-rust.html
[crates-and-modules]: http://doc.rust-lang.org/stable/book/crates-and-modules.html

## Source

 [view source]()
