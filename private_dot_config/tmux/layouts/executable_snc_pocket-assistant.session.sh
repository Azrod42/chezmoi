# Set a custom session root path. Default is `$HOME`.
# Must be called before `initialize_session`.
session_root "$HOME/SnC/mobile/pocket-assistant/"

# Create session with specified name if it does not already exist. If no
# argument is given, session name will be based on layout file name.
if initialize_session "Pocket-assistant"; then

  
  new_window -c "servers"
  select_window 1
  split_h 72
  split_v 30
  run_cmd "cd $HOME/SnC/mobile/pocket-assistant/" 1
  run_cmd "cd $HOME/SnC/mobile/pocket-assistant/" 2
  run_cmd "cd $HOME/SnC/mobile/pocket-assistant/" 3
  run_cmd "npm i" 1
  run_cmd "npm run android" 1
  run_cmd "ccd; api" 2
  run_cmd "vpnoff" 2
  run_cmd "vpn" 2
  run_cmd "yarn" 2
  run_cmd "yarn start:dev" 2
  run_cmd "tmux rename-window Servers" 3
  run_cmd "tmuxifier w code_window" 3
  run_cmd "clear" 3

fi

# Finalize session creation and switch/attach to it.
finalize_and_go_to_session
