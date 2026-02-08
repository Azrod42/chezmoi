# Set a custom session root path. Default is `$HOME`.
# Must be called before `initialize_session`.
session_root "$HOME/SnC/web/megahub/"

# Create session with specified name if it does not already exist. If no
# argument is given, session name will be based on layout file name.
if initialize_session "Megahub"; then
  new_window -c "servers"

split_v 30
split_h 50

run_cmd "tmux rename-window Servers" 3
run_cmd "yarn" 1
run_cmd "ssh -i ~/.ssh/megahub.pem  ec2-user@35.181.4.112" 2
run_cmd "yarn start:dev" 1
run_cmd "tmuxifier w code_window" 3
run_cmd "clear" 3
fi

# Finalize session creation and switch/attach to it.
finalize_and_go_to_session
