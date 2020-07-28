import argparse
import subprocess

text = 'quiketc is a program to keep /etc in a Git repository.'
version = '1.0'

parser = argparse.ArgumentParser(description=text)
parser.add_argument("-i", "--init", help="Creates a git repository for /etc", action='store_true')
parser.add_argument("-I", "--ident", help="Defines username and email for you (Don't run unless you are told to as this overwrites git settings for commits)", action='store_true')
parser.add_argument("-c", "--commit", help="Adds all new files and commits", action='store_true')
parser.add_argument("-C", "--noadd_commit", help="Commits without adding any new files", action='store_true')
parser.add_argument("-E", "--erase", help="Deletes git repository in /etc", action='store_true')
parser.add_argument("-s", "--reset", help="Undoes last commit (to un-undo commits run -s again.)", action='store_true')
parser.add_argument("-H", "--hardreset", help="Undoes last commit permanently (Use -s instead, unless -h is needed or desired)", action='store_true')
parser.add_argument("-V", "--version", help="Prints the version of quiketc.", action='store_true')
args = parser.parse_args()

if args.init:
	print('[quiketc] INFO: Initalising git repository at /etc.')
	try:
		subprocess.check_output("cd /etc && git init .", shell=True)
		print('[quiketc] SUCCESS: /etc is now initialised! Run quiketc with -c to commit to it!')
	except subprocess.CalledProcessError as e:
		print('[quiketc] ERROR: git init failed! Do you have permissions? Do you have git installed?')
		print('[quiketc] INFO: You may be trying to run -i when the Git repository is already initalised, if you are trying to run ident then please use -I or --ident.')
elif args.commit:
	print('[quiketc] INFO: Adding new files and committing.')
	try:
		subprocess.check_output("cd /etc && git add .", shell=True)
		print('[quiketc] INFO: Successfully added new files')
		subprocess.check_output("cd /etc && git commit -a -m \"quiketc commit\"", shell=True)
		print('[quiketc] SUCCESS: Current state of /etc committed!')
	except subprocess.CalledProcessError as e:
		print('[quiketc] ERROR: add or commit failed! Do you have permissions?')
		print('[quiketc] INFO: If you don\'t see \"Successfully added new files\", you may not have any new files and/or changes in /etc. To test try running quiketc with -C or --noadd_commit and see if it fails')
		print('[quiketc] INFO: If the above error is telling you to set global variables like user.name or user.email, please run quiketc with -I or --ident and try committing again.')
elif args.ident:
	print('[quiketc] INFO: Adding identity to account.')
	try:
		subprocess.check_output("git config --global user.email \"quiketc@localhost.local\"", shell=True)
		subprocess.check_output("git config --global user.name \"quiketc\"", shell=True)
		print('[quiketc] SUCCESS: Identity added! You should now be able to commit!')
	except subprocess.CalledProcessError as e:
		print('[quiketc] ERROR: git config failed! Do you have permissions? Do you have git installed?')
elif args.noadd_commit:
	print('[quiketc] INFO: Committing.')
	try:
		subprocess.check_output("cd /etc && git commit -a -m \"quiketc noadd commit\"", shell=True)
		print('[quiketc] SUCCESS: All pre-added files in /etc have been committed!')
	except subprocess.CalledProcessError as e:
		print('[quiketc] ERROR: commit failed! Do you have permissions?')
		print('[quiketc] INFO: If you see only \"Committing.\" above, you probably have no changes.')
		print('[quiketc] INFO: If the above error is telling you to set global variables like user.name or user.email, please run quiketc with -I or --ident and try committing again.')
elif args.reset:
	print('[quiketc] INFO: Softresetting to last commit.')
	try:
		subprocess.check_output("cd /etc && git reset --soft HEAD~1", shell=True)
		print('[quiketc] SUCCESS: Previous commit soft-reversed!')
	except subprocess.CalledProcessError as e:
		print('[quiketc] ERROR: softreset failed! Do you have permissions?')
elif args.hardreset:
	print('[quiketc] WARNING: Are you absolutely sure you want to hardreset? This DELETES the last commit permanently.')
	prompt = input("[quiketc] PROMPT: Are you sure? (Y/N): ")
	if prompt == "Yes" or prompt == "Y" or prompt == "yes" or prompt == "y":
		print('[quiketc] INFO: Hardresetting to last commit. User confirmed request.')
		try:
			subprocess.check_output("cd /etc && git reset --hard HEAD~1", shell=True)
			print('[quiketc] SUCCESS: Previous commit hard-reversed!')
		except subprocess.CalledProcessError as e:
			print('[quiketc] ERROR: hardreset failed! Do you have permissions?')
	else:
		print('[quiketc] ERROR: User rejected prompt. Not hardresetting.')
elif args.erase:
	print('[quiketc] WARNING: Are you absolutely sure you want to erase your /etc git repository? This WON\'T touch your /etc folder and just erase the git data!')
	prompt = input("[quiketc] PROMPT: Are you sure? (Y/N): ")
	if prompt == "Yes" or prompt == "Y" or prompt == "yes" or prompt == "y":
		prompt = input("[quiketc] PROMPT: Are you absolutely positive you want to erase your /etc git repository? (Y/N): ")
		if prompt == "Yes" or prompt == "Y" or prompt == "yes" or prompt == "y":
			print('[quiketc] INFO: Erasing /etc git repository. User confirmed request.')
			try:
				subprocess.check_output("cd /etc && rm -rf .git", shell=True)
				print('[quiketc] SUCCESS: /etc repository erased! Thanks for using quiketc.')
			except subprocess.CalledProcessError as e:
				print('[quiketc] ERROR: erasure failed! Do you have permissions? Is there a repository?')
		else:
			print('[quiketc] ERROR: User rejected prompt. Not erasing /etc repository.')
	else:
		print('[quiketc] ERROR: User rejected prompt. Not erasing /etc repository.')
elif args.version:
	print('[quiketc] INFO: quiketc '+version+' by Colean.')
	print('[quiketc] INFO: https://github.com/Zayne64/quiketc')
else:
	print('[quiketc] ERROR: Argument required! Run quiketc with -h to see the arguments!')
