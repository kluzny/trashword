# Trashword

 "The unhackable<sup>**1**</sup> password manager<sup>**2**</sup>"

## Author's Note

This is a toy. This is not a good way to manage your passwords. Hic sunt dracones.

## Mechanisms

Securely storing passwords is a challenging endeavor, so instead of that we will just generate them. Using a user provide secret we generate a secure hash that is suitable for use as a password. This process is secure-ish, and deterministic. Under the hood we are building salted and hashed composite keys using your provided configuration and secret to generate a strong unreversible password. With minimal configuration data and a strong master key, the potential for password collisions is astronomical.

```bash
$ trashword auth
 master key: <super secret goes here>
e3Cpz2pWeyX8aLjSgEBTGjFSQ0M5xDcQFkHdUMjfWyQ
```

Optionally, send it right to your clipboard with the `-c` or `--clipboard` flag.

```bash
$ trashword auth -c
master key: <super secret goes here>
copied!
```

It is generally recommended to use a unique password for each website or application you might use. You can generate site-specific passwords using the optional flags.

```bash
$ trashword auth --domain example.com
master key: <super secret goes here>
YbkTSHPrMzRF9tq5XCc1D4Yws+PjWk0WVEXqrb0SNDc
```

```bash
$ trashword auth -d some.other.domain.example.com
master key: <super secret goes here>
RqhiMUZOGOwle6Udd148tmz6xvZX+7sh2xNMUuADdto
```

See the output of `trashword auth --help` for more configuration options.

## Use cases

* Literally none. Please do not use this.
* Trivial password generation.
* Ephemeral environments where passwords will be generated systematically and then discarded.

## FAQ

**Can I use this in production?**

Yes, but you should not.

**Stable?**

Literally any minor version, configuration, or argument change will irrevocably modify all of the generated output.

**What hashing algorithm are you using?**

[Argon2id](https://docs.rs/argon2/latest/argon2/), because I think I saw it on the NIST website and it has first class rust support.

**Why did you write this?**

To [learn rust](https://www.rust-lang.org/learn).

**Where should I submit security issues?**

This is highly unlikely. As previously noted<sup>**1**</sup> in our documentation, this is unhackable.

**Help I am being beaten with a [rubber hose](https://en.wikipedia.org/wiki/Rubber-hose_cryptanalysis)!**

This is not really a question and left as an exercise for the reader.

## TODO

* Develop more idiomatic rust
* Better test coverage over the public api
* Support for a wider range of password inputs, e.g. ENV variables, STDIN
* Configuration files similar to sshd_config that can store default flags by host
* Output encodings to allow for password generation with specific characteristics like mixed case, special characters, and other common password format requirements

## Addendum

 \*1 [citation needed]

 \*2 it's more of a generator

Copyright 2023 Kyle Luzny
