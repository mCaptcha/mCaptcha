set -e

launch_kv() {
	bash -c "exec -a kv ./target/debug/kv-test-util $1"
}

kill_kv() {
	kill -9 $(pidof kv)
}

help() {
	cat << EOF
USAGE:
  ./network.sh
  launch		 launches key-value server
  kill			 kills key-value server
EOF
}

if [ -z $1 ]
then
    help
elif [ $1 == 'launch' ]
then 
	if [ -z $2 ]
	then
		help
	else
		launch_kv $2
	fi
elif [ $1 == 'kill' ]
then
		kill_kv
else
	help
fi
