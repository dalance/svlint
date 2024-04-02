Package identifier should have the same name as the file it's in.

```package foo;``` is allowed to live in any file of naming convention ```foo <Non-Identifier> <whatever else>```

According to Clause 5.6 of IEEE 1800-2017:

> A simple identifier shall consist of a sequence of letters, digits, dollar signs (`$`), and underscore (`_`) characters.

Any symbol defined outside this exhaustive list is considered a non-identifier.

The stopping point for string matching has to be a non-identifier character.

For example, the package declaration ```package foo;``` is valid in filenames such as ```foo-Bar.sv```, ```foo.debug.sv```, and ```foo-final-version.sv```. Each of these filenames begins with the package identifier ```foo``` and is immediately followed by a non-identifier character (```-```, ```.```, or another acceptable symbol), making them compliant. A filename like ```FooBar.sv``` is invalid for the ```package Foo;``` declaration since it does not contain a non-identifier character following the package name.

Note that as a consequence, only one package can be declared per file.