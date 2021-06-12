https://doc.rust-lang.org/cargo/reference/config.html

Cargo allows local configuration for a particular package as well as global configuration. It looks for configuration files in the current directory and all parent directories. If, for example, Cargo were invoked in /projects/foo/bar/baz, then the following configuration files would be probed for and unified in this order:

/projects/foo/bar/baz/.cargo/config.toml
/projects/foo/bar/.cargo/config.toml
/projects/foo/.cargo/config.toml
/projects/.cargo/config.toml
/.cargo/config.toml
$CARGO_HOME/config.toml which defaults to:
Windows: %USERPROFILE%\.cargo\config.toml
Unix: $HOME/.cargo/config.toml
With this structure, you can specify configuration per-package, and even possibly check it into version control. You can also specify personal defaults with a configuration file in your home directory.

If a key is specified in multiple config files, the values will get merged together. Numbers, strings, and booleans will use the value in the deeper config directory taking precedence over ancestor directories, where the home directory is the lowest priority. Arrays will be joined together.

Configuration values with sensitive information are stored in the $CARGO_HOME/credentials.toml file. This file is automatically created and updated by cargo login. It follows the same format as Cargo config files.


[registry]
token = "…"   # Access token for crates.io

[registries.<name>]
token = "…"   # Access token for the named registry
Tokens are used by some Cargo commands such as cargo publish for authenticating with remote registries. Care should be taken to protect the tokens and to keep them secret.

As with most other config values, tokens may be specified with environment variables. The token for crates.io may be specified with the CARGO_REGISTRY_TOKEN environment variable. Tokens for other registries may be specified with environment variables of the form CARGO_REGISTRIES_<name>_TOKEN where <name> is the name of the registry in all capital letters.

