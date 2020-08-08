# jde
Joshua's Desktop Environment

## Motivation
Modern Unix systems are giant balls of mutable state. For the most part, this
is fine, but it leads to problems when trying to reproduce development
environments across multiple machines. For years, my approach to this problem
was to write elaborate shell-scripts that worked 60% of the time, but which
always broke for some subtle reason or another. I was never really satisfied.

Enter the `nix` package manager, which solves this problem completely. There's
a learning curve, but persevere and the rewards are great: You can manage your
system with code in a single git repository! There're a few errant config files
for programs like `emacs` that need to be managed somewhat differently, but
once [home manager](https://github.com/rycee/home-manager) gets more stable,
you'll be able to track them with `nix` as well.

## Dependencies
- Linux
- Git
- Nix

...that's it! `nix` will install everything else you need.

## Installing

Run this one command:

```
nix-env -ir -f install.nix
```

Done! This repository is now your single-source of truth. 🥳
