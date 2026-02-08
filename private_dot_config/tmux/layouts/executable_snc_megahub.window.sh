window_root "$HOME/SnC/web/megahub/"

new_window "snc_megahub"

split_h 20
split_v 50

run_cmd "yarn" 1
run_cmd "ssh -i ~/.ssh/megahub.pem  ec2-user@35.181.4.112" 2
run_cmd "yarn start:dev" 1
