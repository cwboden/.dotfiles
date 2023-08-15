function start_ssh_agent() {
	# Check if connection is open to authentication agent
	ssh-add -l &>/dev/null
	if [[ $? == 2 ]]; then
		if [[ ! -e ~/.ssh-agent ]]; then
			touch ~/.ssh-agent
		fi

		# If not, load stored agent connection info
		test -r ~/.ssh-agent && eval "$(<~/.ssh-agent)" >/dev/null

		# Check if it's connected now
		ssh-add -l &>/dev/null
		if [[ $? == 2 ]]; then
			# If not, start agent and store connection info
			echo "SSH Agent not running, starting..."
			(
				umask 066
				ssh-agent >~/.ssh-agent
			)
			eval "$(<~/.ssh-agent)" >/dev/null
		fi
	fi

	# Agent should be running now, load identities if they aren't present
	ssh-add -l &>/dev/null
	if [[ $? == 1 ]]; then
		echo "SSH Agent needs to add identities:"
		# Agent doesn't have identities, so add them
		ssh-add -t 8h
	fi
}
