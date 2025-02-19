# tiny-ver

> A **tiny** version parser with a big personality!

## 🌟 Features

- **Simple Parsing:** Easily convert a version string (e.g., `"1.2.3"` or `"1.2.3-rc.1"`) into a `TinyVersion`.
- **Strict Pre-release Validation:** Pre-release identifiers are rigorously checked:
  - Dot-separated identifiers.
  - No empty identifiers.
  - Only ASCII alphanumeric characters and hyphens allowed.
  - Numeric identifiers cannot have leading zeros (unless they are exactly `"0"`).
- **Helper Methods:** Generate versioned names for your projects with a simple method.

## 🚀 Installation

Add `tiny-ver` to your `Cargo.toml`:

```toml
[dependencies]
tiny-ver = "0.1.0"
```

## About Contributions

This is an open source project with a single copyright holder.
While the code is publicly available under [LICENSE](LICENSE), I am not accepting external contributions at this time.

You are welcome to:

- Use the code according to the license terms
- Fork the project for your own use, following the license terms
- Report issues
- Provide feedback
- Share the project

If you have suggestions or find bugs, please feel free to open an issue for discussion.
While I cannot accept pull requests, I value your feedback and engagement with the project.

Thank you for your understanding and interest in the project! 🙏

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

_Copyright (c) 2025 Peter Bjorklund. All rights reserved._
