Meno (MEdia AnNOtator)
======================

A simple tool for extracting media file annotations from text and formatting
them either for either human or machine consumption.  This means pretty printing
as text, or formatting as json, csv, or other formats that make it easy to
process with UNIX commands.

I may also add the ability to annotate a media file interactively if I can find
a simple enough way to do it.  The current thinking is that it will wrap mpv if
it is present on the system.

I may also want to play the media file back from a given mark. A basic shell
script using `mpv --start=<mark>`, `awk` and `fzf` will probably suffice for
that, but I may make it possible to integrate such a script into the app.


## Format Specification ##

The format for Meno annotations is as follows:

A mark is a string matching the format `[hh:][[m]m:][s]s[.xxx]`

Examples of a mark might be:
- 5          (5s)
- 5.002      (5s 2ms)
- 2:5        (2m 5s)
- 3:24.088   (3m 24s 88ms)
- 1:30:45    (1h 30m 45s)

The spec also leaves room for the definition of a range as being the time
between 2 marks, but I don't think this will actually be necessary so it is
left out for now.  More likely, I will use "unitl <mark>" as an indicator
of range in the body of the annotation istself.

A mark placed at the very beginning of a line represents the beginning of an
annotation.  All text after the mark represents the body of the annotation
itself.  A mark may be placed within the body of an annotation, in which case
it will be considered a part of the annotation and not starting a new one.

If you wish to refer to a mark within the body of an annotation and it happens
to start on a new line, you can place one or more whitespace characters in
front of the mark. Whitespace will be ignored when formatting an annotation, so
you can pretty print annotations where you write them.

An annotation can span multiple lines if the subsequent lines are prefixed with
one or more whitespace characters.

If a line does not begin with whitespace or a mark, then it must be a comment
and is ignored along with all following lines that do not begin with a mark.

An example of a set of meno-compliant annotations might be the following

```
0:07    Introductions
0:37    Presentation of what will be discussed this episode.
 Annotations can span multiple lines. Just remember to indent the text.
13:00   Plosives until 13:37
14:07   Multiline annotations can be
        indented however you like.
        Formatting can be preserved by Meno.
        Here's a random quote:
            Never interrupt your enemy while he's making a mistake.
            ~ Napoleon
            
20:22
  In this example, the mark is put on its own line and the body of the annotation
  placed underneath.  We know it's the body because it is indented.
  20:48 isn't a mark because it's also indented.  That means it's a part of the
  body.
  
This is not a part of the annotation at all because it is not indented and so
it will be ignored by Meno.

25:05.325
  ## Markdown Example
  
  This annotation is formatted with markdown. It's just text, so it can contain
  whatever you like.  As long as each line is proceeded by whitespace, it will
  be considered a part of the annotation for the proceeding mark.
  
1:13:45 Interrupted by your mum.
```
