# Ignore duplicates
setopt hist_ignore_dups
setopt hist_ignore_all_dups

# Remove useless blanks
setopt hist_reduce_blanks

# Append to the $HISTFILE immediately instead of when the shell exits
setopt inc_append_history

# Expand globs to '' rather than '*.*' if there are no matches
setopt null_glob

# Size history accordingly
export HISTSIZE=10000
export SAVEHIST=10000
export HISTFILE=~/.history

# Better history
# https://coderwall.com/p/jpj_6q/zsh-better-history-searching-with-arrow-keys
autoload -U up-line-or-beginning-search
autoload -U down-line-or-beginning-search
zle -N up-line-or-beginning-search
zle -N down-line-or-beginning-search
bindkey "^[[A" up-line-or-beginning-search   # Up
bindkey "^[[B" down-line-or-beginning-search # Down
