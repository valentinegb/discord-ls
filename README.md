# Discord LS

A language server that gives any [LSP] code editor Discord integration.

... That's the idea at least. Right now, this is only a proof of concept. It was
born out of necessity: I wanted Discord integration for [Zed], but its extension
capabilities are currently limited. [Zed], like most extensible editors, adheres
to the [Language Server Protocol][LSP] as a client, and making a language server
that just reports info about what you're editing to Discord, while indeed a
workaround and not ideal, *works!* But the whole point of [LSP] is to be able to
extend a *wide range* of editors, so I want to do that with this project, to be
able to add Discord integration to your favorite editor, whatever that may be. I
haven't managed that yet though, this is only a very simple proof of concept
that shows your editor as "Zed" no matter what it actually is and only supports
Rust and TOML files.

[LSP]: https://microsoft.github.io/language-server-protocol/
[Zed]: https://zed.dev
