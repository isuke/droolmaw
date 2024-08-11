prompt_precmd() {
  PROMPT=`./target/release/droolmaw`
  RPROMPT=`./target/release/droolmaw --right`
}

add-zsh-hook precmd prompt_precmd
