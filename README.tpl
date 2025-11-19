# {{crate}}

[![Docs](https://docs.rs/ansiconst/badge.svg)](https://docs.rs/ansiconst) [![Crates.io](https://img.shields.io/crates/v/ansiconst.svg)](https://crates.io/crates/ansiconst)

## Contents

* [Motivation](#Motivation)
* [Key Features](#Key-Features)
* [Examples](#Examples-3)
* [Upgrading from v0.1.x](#upgrading-from-v01x)
* [Version History](#Version-History)
* [Licence](#Licence)

{{readme}}

[`print!`]:              https://doc.rust-lang.org/std/macro.print.html
[`println!`]:            https://doc.rust-lang.org/std/macro.println.html
[`eprint!`]:             https://doc.rust-lang.org/std/macro.eprint.html
[`eprintln!`]:           https://doc.rust-lang.org/std/macro.eprintln.html
[`write!`]:              https://doc.rust-lang.org/std/macro.write.html
[`writeln!`]:            https://doc.rust-lang.org/std/macro.writeln.html
[`format!`]:             https://doc.rust-lang.org/std/macro.format.html
[`format_args!`]:        https://doc.rust-lang.org/std/macro.format_args.html
[`thread_local!`]:       https://doc.rust-lang.org/std/macro.thread_local.html

[`Ansi`]:                https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html
[`Color`]:               https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html
[`Effect`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html
[`Styled<T>`]:           https://docs.rs/ansiconst/latest/ansiconst/struct.Styled.html
[`StyledString`]:        https://docs.rs/ansiconst/latest/ansiconst/struct.StyledString.html
[`styled!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.styled.html
[`styled_format!`]:      https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format.html
[`styled_format_args!`]: https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format_args.html
[`styled_write!`]:       https://docs.rs/ansiconst/latest/ansiconst/macro.styled_write.html
[`styled_writeln!`]:     https://docs.rs/ansiconst/latest/ansiconst/macro.styled_writeln.html
[`paint!`]:              https://docs.rs/ansiconst/latest/ansiconst/macro.paint.html
[`paintln!`]:            https://docs.rs/ansiconst/latest/ansiconst/macro.paintln.html
[`epaint!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.epaint.html
[`epaintln!`]:           https://docs.rs/ansiconst/latest/ansiconst/macro.epaintln.html
[Color::num]:            https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html#method.num
[Color::rgb]:            https://docs.rs/ansiconst/latest/ansiconst/enum.Color.html#method.rgb
[Effect::not]:           https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html#method.not
[`Ansi::empty()`]:       https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.empty
[Ansi::important]:       https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.important
[Ansi::only]:            https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.only
[Ansi::no_ansi]:         https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.no_ansi

## Version History

<table>
<thead>
<tr>
    <th>Version</th>
    <th>Date</th>
    <th>Comments</th>
</tr>
</thead>
<tbody>
<tr>
    <td>v0.2.1</td>
    <td>19-Nov 2025</td>
    <td>Enhancement release
        <ul>
        <li>Add <code>important</code> modifier (similar to CSS <code>!important</code>)</li>
        <li>Add <code>StyledString</code> to allow overriding ANSI styles inside plain Strings</li>
        <li>Automatically import colours/effects inside macros</li>
        <li>Add improved introspection API</li>
        <li>Improve test coverage</li>
        <li>Fix some edge cases where suboptimal ANSI codes were emitted to transition between styles</li>
        <li>Fix not treating identical 4-bit/8-bit/rgb colors as equal</li>
        <li>Rename <code>Colour</code> to <code>Color</code></li>
        </ul>
    </td>
</tr>
<tr>
    <td>v0.1.1</td>
    <td>10-Jun 2024</td>
    <td>Bugfix/docs release
        <ul>
        <li>Fix handling of <code>FORCE_COLOR</code>, <code>NO_COLOR</code> env vars</li>
        <li>Fix missing <code>fn</code> for creating <code>AnsiWriter</code> instances</li>
        <li>Fix broken links in <code>README</code></li>
        <li>Add more explanation to Rustdoc</li>
        </ul>
    </td>
</tr>
<tr>
    <td>v0.1.0</td>
    <td>09-Jun 2024</td>
    <td>Initial release</td>
    </td>
</tr>
</tbody>
</table>

## Licence
{{license}}
