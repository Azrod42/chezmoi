# Set a custom session root path. Default is `$HOME`.
# Must be called before `initialize_session`.
session_root "$HOME/Documents/Cealum/"

# Create session with specified name if it does not already exist. If no
# argument is given, session name will be based on layout file name.
if initialize_session "cealum"; then
  new_window -c "code"
  select_window 1
  split_h 45
  split_v 60
  run_cmd "cd OpusCealum-api" 1
  run_cmd "cd OpusCealum" 2
  run_cmd "yarn" 1
  run_cmd "yarn" 2
  run_cmd "yarn start:dev" 1
  run_cmd "yarn dev" 2
  select_pane 3
  run_cmd "tmux rename-window Servers" 3
  run_cmd "tmuxifier w code_window" 3
  select_window 1
  run_cmd "clear" 3
fi

# Finalize session creation and switch/attach to it.
finalize_and_go_to_session


