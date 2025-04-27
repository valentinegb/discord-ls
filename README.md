# Discord LS

A language server that gives any [LSP] code editor Discord integration.

... That's the idea at least. Right now, this is only a proof of concept. It was
born out of necessity: I wanted Discord integration for [Zed], but its extension
capabilities are currently limited. [Zed], like most extensible editors, adheres
to the [Language Server Protocol][LSP] as a client, and making a language server
that just reports info about what you're editing to Discord, while indeed a
workaround and not ideal, *works!* [Here's the Zed extension]. But the whole
point of [LSP] is to be able to extend a *wide range* of editors, so I want to
do that with this project, to be able to add Discord integration to your
favorite editor, whatever that may be. I haven't managed that yet though, this
is only a very simple proof of concept that shows your editor as "Zed" no matter
what it actually is and only supports Rust and TOML files.
<br />
<br />
<p align="center">
  <img width="285" alt="Screenshot 2025-04-26 at 8 21 55 PM" src="https://github.com/user-attachments/assets/0f9ce28a-f7de-42df-bb3b-36ca44c849cf" />
</p>
<p align="center">
  <i>When the language server first initializes, it has no way of knowing the currently open file.</i>
</p>
<br />
<p align="center">
  <img width="285" alt="Screenshot 2025-04-26 at 8 22 16 PM" src="https://github.com/user-attachments/assets/37eaaa32-29fe-4b5f-9d00-72d12f4c7201" />
</p>
<p align="center">
  <i>Editing a Rust file.</i>
</p>
<br />
<p align="center">
  <img width="285" alt="Screenshot 2025-04-26 at 8 22 28 PM" src="https://github.com/user-attachments/assets/36e40694-637d-4b45-b0ec-f092c4c6da47" />
</p>
<p align="center">
  <i>Editing a TOML file.</i>
</p>

[LSP]: https://microsoft.github.io/language-server-protocol/
[Zed]: https://zed.dev
[Here's the Zed extension]: https://github.com/valentinegb/zed-discord
