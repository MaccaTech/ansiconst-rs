# {{crate}}

### [View the Rustdoc](https://docs.rs/ansiconst)

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
[`Colour`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html
[`Effect`]:              https://docs.rs/ansiconst/latest/ansiconst/enum.Effect.html
[`Styled<T>`]:           https://docs.rs/ansiconst/latest/ansiconst/struct.Styled.html
[`styled!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.styled.html
[`styled_format!`]:      https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format.html
[`styled_format_args!`]: https://docs.rs/ansiconst/latest/ansiconst/macro.styled_format_args.html
[`styled_write!`]:       https://docs.rs/ansiconst/latest/ansiconst/macro.styled_write.html
[`styled_writeln!`]:     https://docs.rs/ansiconst/latest/ansiconst/macro.styled_writeln.html
[`paint!`]:              https://docs.rs/ansiconst/latest/ansiconst/macro.paint.html
[`paintln!`]:            https://docs.rs/ansiconst/latest/ansiconst/macro.paintln.html
[`epaint!`]:             https://docs.rs/ansiconst/latest/ansiconst/macro.epaint.html
[`epaintln!`]:           https://docs.rs/ansiconst/latest/ansiconst/macro.epaintln.html
[Colour::Ansi256]:       https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html#variant.Ansi256
[Colour::Rgb]:           https://docs.rs/ansiconst/latest/ansiconst/enum.Colour.html#variant.Rgb
[Ansi::protect_attrs]:   https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.protected_attrs
[Ansi::only]:            https://docs.rs/ansiconst/latest/ansiconst/struct.Ansi.html#method.only

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

## License
{{license}}
