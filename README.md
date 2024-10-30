# Droolmaw [![git-consistent friendly](https://img.shields.io/badge/git--consistent-friendly-brightgreen.svg)](https://github.com/isuke/git-consistent)  [![MIT](https://img.shields.io/github/license/mashape/apistatus.svg)](https://raw.githubusercontent.com/isuke/droolmaw/main/LICENSE)

![](https://raw.githubusercontent.com/isuke/droolmaw/images/image01.png)

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
name = "Id"
color = "Magenta"

[[l_components_first]]
name = "DirPath"
color = "Blue"

[[l_components_second]]
name = "DateTime"
color = "White"
ng_color = "Red"

[[l_components2]]
name = "None"
color = "White"

[[r_components]]
name = "Langs"
color = "Blue"
langs = ["rust"]
```

### 2. Download and Set

#### A. By [zinit](https://github.com/zdharma-continuum/zinit)

```sh
zinit ice from"gh-r" as"program"
zinit load "isuke/droolmaw"

prompt_precmd() {
  export DROOLMAW_RETVAL=$?
  PROMPT=`droolmaw`
  PROMPT2=`droolmaw --2`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```

#### B. Compile by Yourself

```sh
git clone https://github.com/isuke/droolmaw.git
cd droolmaw
cargo build --release
export PATH="$PATH:/your/path/droolmaw/target/release"

prompt_precmd() {
  export DROOLMAW_RETVAL=$?
  PROMPT=`droolmaw`
  PROMPT2=`droolmaw --2`
  RPROMPT=`droolmaw --right`
}
add-zsh-hook precmd prompt_precmd
```

## Spec of Setting File

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

### key: `name`

| value                           | option           | description              |
| ------------------------------- | ---------------- | ------------------------ |
| None                            |                  | none                     |
| Id                              |                  | id (user name)           |
| Dir                             |                  | current directory name   |
| DirPath                         | max_length = 123 | current directory path   |
| DateTime                        |                  | date time                |
| GitName                         |                  | git author name          |
| GitCurrentBranchAndStatuses     |                  | git current branch name and statuses |
| GitRemotesAndStatuses           |                  | git remote name and statues          |
| Langs                           | langs = ["ruby", "node", "rust", "python"] | The version of the language managed by [mise](https://github.com/jdx/mise) in the current directory |
| ResultText                      | ok_text = "ok" ng_text = "ng" | if prev command's exit code is 0 => print ok_text else => print ng_text |

### key: `color` and `ng_color`

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

The `color` is background color.
Background color is the `ng_color` instead of the `color` if prev command's exit code is not 0.
