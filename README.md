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

You can even load context from a yamlfile with the `-f` flag:

```
$ cat context.yaml
some_input: Hello
$ cat template.hbs
output = {{some_input}}
$ renderbars -f context.yaml template.hbs
output = Hello
```

Variables passed on the command line override variables in the context file.
