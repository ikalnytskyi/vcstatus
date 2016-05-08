# vcstatus

vcstatus is a command line tool that prints a short string with VCS
information about the current working directory. A main use case is
to make shell prompts consume that information, so you never forget
current VCS and active branch.

vcstatus is designed keeping in mind the main use case, so it was
crucial to have a fast tool. That means it can't be written in
scripting language (like Python), even if I'd prefer to. So I choose
Rust just because I wanted to learn it for a while.

```bash
$ vcstatus -f "[%n %b]"
[git master]
```

## VCS

* Git
* Mercurial

## Formats

* `%n` - prints VCS short name
* `%b` - prints VCS active branch

## Links

* Source: <https://github.com/ikalnitsky/vcstatus>
* Bugs: <https://github.com/ikalnitsky/vcstatus/issues>
