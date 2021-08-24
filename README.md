# Renderbars

I write a lot of things that feel like they should be generated automatically
by a template engine. Renderbars is an application to render input files as
Handlebars templates, taking context on the command line, and writing the
resulting contents to an output file.

# Usage

Context is passed with the `-c` flag, which can appear on a single invocation
of the tool as many times as you want. It's important that context takes the
form `name=val`, where `name` is the name of the field, and `val` is its value.

```
```

You can pass Handlebars templates as the value, and there are even some custom
helpers to do useful things:

```
# The stdin helper reads the value of the field "somevar" from stdin, until EOF
# is reached.
$ renderbars -c "somevar={{stdin}}" input.hbs

# Use \ to escape:
$ renderbars -c "somevar=\{{rendered_verbatim}}" input.hbs
```

The second form is important to know, because Renderbars will throw an error if
the template accesses a field not provided in the context. One can use this
form to partially render a template, like so:

```
# Generate the intermediate template
$ renderbars -c "foo=bar" -c "baz=\{{baz}}" input.hbs intermediate.hbs

# ...do some things to generate the value of "baz"...

$ renderbars -c "baz=valueOfBaz" intermediate.hbs
```

You can even load context from a file with the `-f` flag. The format for this
file is very simple:

```
# Lines starting with '#' are ignored...
    # Even if there's white space in front

# Set variables:
var_name=50

# Or...
var_name = 50

# INVALID: Comments are not valid on the same lines as context
does_not_work = 50 # NOT VALID!!!!
```

Variables passed on the command line override variables in the context file.
