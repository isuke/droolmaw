prompt_precmd() {
  PROMPT=`./target/release/droolmaw`
}

add-zsh-hook precmd prompt_precmd
