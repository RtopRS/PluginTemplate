<!--suppress HtmlDeprecatedAttribute -->
<h1 align="center">
  PluginTemplate
</h1>
<p align="center">
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
    </a>
    <a href="https://github.com/RtopRS/PluginTemplate">
        <img src="https://img.shields.io/badge/Git-F05032?style=for-the-badge&logo=git&logoColor=white" alt="Use git">
    </a>
    <br>
    <a href="https://github.com/RtopRS/PluginTemplate/blob/main/LICENSE">
        <img src="https://img.shields.io/github/license/RtopRS/PluginTemplate?style=for-the-badge" alt="License">
    </a>
    <a href="https://github.com/RtopRS/PluginTemplate/stargazers">
        <img src="https://img.shields.io/github/stars/RtopRS/PluginTemplate?style=for-the-badge" alt="Stars">
    </a>
</p>
<h3 align="center">
    <strong>A simple Rtop plugin template.</strong>
</h3>

## Important
This repository only serves as an example plugin, it's pretty useless otherwise since it only displays a basic "Hello from Rtop!".
<br>
**It is therefore completely useless to open a PR whatever the nature of the change, in case you open a PR it will be closed immediately.**
<br>

## Usage
If you want to start the development of a plugin you can clone this repository to be able to modify it as you like.

Start by cloning:
```bash
git clone https://github.com/RtopRS/PluginTemplate.git
```

Then you can change the git commit URL to use your own git repo:
```bash
git remote set-url origin new-git-url
```

**For the next step you need to have Rust and Cargo installed on your PC, for that follow the [official documentation](https://www.rust-lang.org/tools/install).**

Now switch to project folder and compile:
```bash
cd PluginTemplate && cargo build
```

For the development phase you can build in debug mode:
```bash
cargo build
```

You just have to modify the `Cargo.toml` file, once this is done you can start developing your plugin!

## License
**[PluginTemplate](https://github.com/RtopRS/PluginTemplate) | [Mozilla Public License 2.0](https://github.com/RtopRS/PluginTemplate/blob/main/LICENSE)**