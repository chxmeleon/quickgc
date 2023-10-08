# QuickGC: Your Quick Git Commit Assistant

QuickGC is crafted for developers who seek a streamlined, interactive, and efficient way to commit their code. Say goodbye to the hassle of typing out commit messages and embrace a world where git commits are quick, styled, and consistent.

## 🚀 Getting Started

### Installation

No frills, no fuss. Installing QuickGC is as simple as running one command:

```sh
cargo install quickgc
```

Or, install it from the source by cloning this repository:

```sh
git clone https://github.com/chxmeleon/quickgc-rust.git
cd quickgc-rust
cargo install --path .
```

### Usage

Initiate QuickGC, and let it guide you through the rest. It’s interactive, intuitive, and user-friendly.

```sh
quickgc
```

## 🎨 Commit Styles at Your Fingertips

QuickGC isn’t just about speed; it’s about style too. Choose from a variety of predefined commit styles, each tailored for a specific type of commit:

- `[FEATURE]`: Introduce a brand-new feature.
- `[BUGFIX]`: Squash those pesky bugs.
- `[BUILD]`: Changes in build processes or dependencies.
- `[STYLE]`: Beautify your code with style adjustments.
- `[REFACTOR]`: Revamp your code without altering its behavior.
- `[DOCS]`: Enhance or create documentation.
- `[TEST]`: Add tests ensuring your code’s reliability.

## ⚙️ Make It Yours

Every project is unique, and QuickGC adapts. Modify the `config.json` file to define your own commit styles, making QuickGC a personalized commit assistant.

```
{
  "prefixes": [
    "[FEATURE]",
    "[BUILD]",
    "[BUGFIX]"
    // ... unleash your creativity
  ]
}
```

## 💡 Contributing

Your insights and skills can make QuickGC even better. Feel free to open issues for bugs or feature requests, and pull requests are always welcome.

## 📜 License

QuickGC is open-source and is generously offered under the MIT License. Check out the [LICENSE](LICENSE) file for more details.
