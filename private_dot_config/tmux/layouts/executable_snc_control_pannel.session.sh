session_root "$HOME/SnC/web/control-panel"

if initialize_session "Control-pannel"; then
  new_window -c "servers"
  select_window 1
  split_h 40
  split_v 70
  run_cmd "cd $HOME/SnC/mobile/control-panel/" 1
  run_cmd "cd $HOME/SnC/mobile/control-panel/" 2
  run_cmd "cd $HOME/SnC/mobile/control-panel/" 3
  run_cmd "nvm use 16.19.0" 1
  run_cmd "npm i --force" 1
  run_cmd "npx ng serve" 1
  # run_cmd "export PATH=${PATH}:$(realpath node_modules/.bin)" 1
  run_cmd "ssh -NT -vv -L "4242:localhost:9442" sc@ceos-1012.local" 2
  run_cmd "xdg-open http://localhost:4200" 3
  run_cmd "tmux rename-window Servers" 3
  run_cmd "tmuxifier w code_window" 3
  run_cmd "clear" 3
fi

finalize_and_go_to_session
