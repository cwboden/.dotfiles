tmux-dev() {
    # Initial windows/panes setup
    tmux new-session -d
    tmux rename-window 'terminal'
    tmux splitw -h
    tmux new-window -d -n 'code' 'vim'
    tmux attach-session -d
}
