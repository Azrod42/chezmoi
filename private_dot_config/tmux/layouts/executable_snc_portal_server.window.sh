# Set window root path. Default is `$session_root`.
# Must be called before `new_window`.
window_root "~/SnC/web/"

# Create new window. If no argument is given, window name will be based on
# layout file name.
new_window "servers"
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
  select_pane 3

