# Droolmaw [![git-consistent friendly](https://img.shields.io/badge/git--consistent-friendly-brightgreen.svg)](https://github.com/isuke/git-consistent)  [![MIT](https://img.shields.io/github/license/mashape/apistatus.svg)](https://raw.githubusercontent.com/isuke/droolmaw/main/LICENSE)

**Droolmaw** is zsh theme.

You need [Nerd Fonts](https://www.nerdfonts.com/).

The following fonts are recommended for Japanese.

- [白源 (はくげん／HackGen)](https://github.com/yuru7/HackGen)
- [PlemolJP](https://github.com/yuru7/PlemolJP)

## How to Install

### 1. Prepare Setting File

Put the following file in your home directory as '.droolmaw.toml'.

```toml
l_separator = ""
r_separator = ""

[[l_components_first]]
name = "id"
color = "Magenta"

[[l_components_first]]
name = "dir_path"
color = "Blue"

[[l_components_second]]
name = "date_time"
color = "White"

[[r_components]]
name = "langs"
color = "Blue"
langs = ["rust"]
```

### 2. Download and Set

#### By [zinit](https://github.com/zdharma-continuum/zinit)

```sh
zinit ice from"gh-r" as"program"
zinit load "isuke/droolmaw"

prompt_precmd() {
  PROMPT=`droolmaw`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```

## Setting File Spec

### key: name

| value                           | option           | description              |
| ------------------------------- | ---------------- | ------------------------ |
| id                              |                  | [TODO]                   |
| dir                             |                  | current directory name   |
| dir_path                        | max_length = 123 | current directory path   |
| date_time                       |                  | date time                |
| git_name                        |                  | git author name          |
| git_current_branch_and_statuses |                  | [TODO]                   |
| git_remotes_and_statuses        |                  | [TODO]                   |
| langs                           | langs = ["ruby", "node", "rust", "python"] | |

### key: color

| value     |
| --------- |
|  Black    |
|  Blue     |
|  Cyan     |
|  Green    |
|  Magenta  |
|  Red      |
|  White    |
|  Yellow   |
