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

[[l_components2]]
name = "none"
color = "White"

[[r_components]]
name = "langs"
color = "Blue"
langs = ["rust"]
```

### 2. Download and Set

#### A. By [zinit](https://github.com/zdharma-continuum/zinit)

```sh
zinit ice from"gh-r" as"program"
zinit load "isuke/droolmaw"

prompt_precmd() {
  PROMPT=`droolmaw`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```

#### B. Compile Yourself

```sh
git clone https://github.com/isuke/droolmaw.git
cd droolmaw
cargo build --release
export PATH="$PATH:/your/path/target/release/droolmaw"

prompt_precmd() {
  PROMPT=`droolmaw`
  PROMPT2=`droolmaw --2`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```

## Setting File Spec

### key: l_separator and r_separator

- hard_divider
    - l_separator: `""` (U+E0B0)
    - r_separator: `""` (U+E0B2)
- triangle1
    - l_separator: `""` (U+E0B8)
    - r_separator: `""` (U+E0BE)
- triangle2
    - l_separator: `""` (U+E0BC)
    - r_separator: `""` (U+E0BA)
- half_circle_thick
    - l_separator: `""` (U+E0B4)
    - r_separator: `""` (U+E0B6)
- flame_thick
    - l_separator: `""` (U+E0C0)
    - r_separator: `""` (U+E0C2)
- ice_waveform
    - l_separator: `""` (U+E0C8)
    - r_separator: `""` (U+E0CA)

### key: name

| value                           | option           | description              |
| ------------------------------- | ---------------- | ------------------------ |
| none                            |                  | none                     |
| id                              |                  | id (user name)           |
| dir                             |                  | current directory name   |
| dir_path                        | max_length = 123 | current directory path   |
| date_time                       |                  | date time                |
| git_name                        |                  | git author name          |
| git_current_branch_and_statuses |                  | git current branch name and statuses |
| git_remotes_and_statuses        |                  | git remote name and statues          |
| langs                           | langs = ["ruby", "node", "rust", "python"] | The version of the language managed by [mise](https://github.com/jdx/mise) in the current directory |

### key: color

| value     |
| --------- |
| Black     |
| Blue      |
| Cyan      |
| Green     |
| Magenta   |
| Red       |
| White     |
| Yellow    |
