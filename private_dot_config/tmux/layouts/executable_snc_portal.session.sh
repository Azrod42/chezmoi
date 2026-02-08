# Set a custom session root path. Default is `$HOME`.
# Must be called before `initialize_session`.
session_root "$HOME/SnC/web/"

# Create session with specified name if it does not already exist. If no
# argument is given, session name will be based on layout file name.
if initialize_session "snc_portal"; then
  new_window -c "servers"
  select_window 1
  split_h 45
  split_v 60
  run_cmd "cd api" 1
  run_cmd "cd portal" 2
  run_cmd "vpnoff" 1
  run_cmd "vpn" 1
  run_cmd "yarn" 1
  run_cmd "yarn" 2
  run_cmd "yarn start:dev" 1
  run_cmd "yarn dev" 2
  run_cmd "tmux rename-window Servers" 3
  run_cmd "tmuxifier w code_window" 3
  run_cmd "clear" 3
  # load_window "code_window"
fi

# Finalize session creation and switch/attach to it.
finalize_and_go_to_session


