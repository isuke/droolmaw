# Droolmaw [![git-consistent friendly](https://img.shields.io/badge/git--consistent-friendly-brightgreen.svg)](https://github.com/isuke/git-consistent)  [![MIT](https://img.shields.io/github/license/mashape/apistatus.svg)](https://raw.githubusercontent.com/isuke/droolmaw/main/LICENSE)

**Droolmaw** is zsh theme.

You need [Nerd Fonts](https://www.nerdfonts.com/).

The following fonts are recommended for Japanese.

- [白源 (はくげん／HackGen)](https://github.com/yuru7/HackGen)
- [PlemolJP](https://github.com/yuru7/PlemolJP)

## How to Install

### By [zinit](https://github.com/zdharma-continuum/zinit)

```sh
zinit ice from"gh-r" as"program"
zinit load "isuke/droolmaw"

prompt_precmd() {
  PROMPT=`droolmaw`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```
