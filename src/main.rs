extern crate clap;
use clap::{Arg, App};
use std::io::{self, Write, Stdin, Read};
use std::process::Command;

fn main() {
	let matches = App::new("quiketc")
		.version("2.0")
        .author("Colean <colean@colean.cc>")
        .about("quiketc is a program to keep /etc in a Git repository.")
        .arg(Arg::with_name("init")
	             .short("i")
	             .long("init")
	             .help("Creates a git repository for /etc"))
	    .arg(Arg::with_name("ident")
			     .short("I")
			     .long("ident")
			     .help("Defines username and email for you (Don't run unless you are told to as this overwrites git settings for commits)"))
		.arg(Arg::with_name("commit")
			     .short("c")
			     .long("commit")
			     .help("Adds all new files and commits."))
		.arg(Arg::with_name("noadd_commit")
			     .short("C")
			     .long("noadd_commit")
			     .help("Commits without adding any new files."))
		.arg(Arg::with_name("version")
			     .short("V")
			     .long("version")
			     .help("Prints the version of quiketc."))
		.arg(Arg::with_name("erase")
				 .short("E")
				 .long("erase")
				 .help("Deletes git repository in /etc."))
		.arg(Arg::with_name("reset")
		          .short("s")
		          .long("reset")
		          .help("Undoes last commit (to un-undo commits run -s again.)"))
	    .arg(Arg::with_name("hardreset")
		          .short("H")
		          .long("hardreset")
		          .help("Undoes last commit permanently (Use -s instead, unless -H is needed or desired)"))
		.get_matches();

	if matches.is_present("init"){
		println!("[quiketc] INFO: Initalising git repository at /etc.");
		let output = Command::new("git")
			.args(&["init"])
			.current_dir("/etc")
			.output()
			.expect("[quiketc] ERROR: git init failed! Do you have permissions? Do you have git installed?");
		if output.status.success(){
			println!("[quiketc] SUCCESS: /etc is now initialised! Run quiketc with -c to commit to it!");
		}else{
			println!("[quiketc] ERROR: git init failed! Do you have permissions? Do you have git installed?");
		}
	}
	else if matches.is_present("ident"){
		println!("[quiketc] INFO: Adding identity to account.");
		let output = Command::new("git")
			.args(&["config", "--global", "user.email", "\"quiketc@localhost.local\""])
			.output()
			.expect("[quiketc] ERROR: git config failed! Do you have permissions? Do you have git installed?");
		if output.status.success(){
			let output1 = Command::new("git")
				.args(&["config", "--global", "user.email", "\"quiketc@localhost.local\""])
				.output()
				.expect("[quiketc] ERROR: git config failed! Do you have permissions? Do you have git installed?");
			if output1.status.success(){
				println!("[quiketc] SUCCESS: Identity added! You should now be able to commit!");
			}else{
				println!("[quiketc] ERROR: git config failed! Do you have permissions? Do you have git installed?");
			}
		}else{
			println!("[quiketc] ERROR: git config failed! Do you have permissions? Do you have git installed?");
		}
	}
	else if matches.is_present("commit"){
		println!("[quiketc] INFO: Adding new files and committing.");
		let output = Command::new("git")
			.args(&["add", "."])
			.current_dir("/etc")
			.output()
			.expect("[quiketc] ERROR: git add failed! Do you have permissions? Do you have git installed?");
		if output.status.success(){
			let output1 = Command::new("git")
				.args(&["commit", "-am","\"quiketc commit\""])
				.current_dir("/etc")
				.output()
				.expect("[quiketc] ERROR: git commit failed! Do you have permissions? Do you have git installed?");
			if output1.status.success(){
				println!("[quiketc] SUCCESS: Current state of /etc committed!");
			}else{
				println!("[quiketc] ERROR: git commit failed! Do you have permissions? Do you have git installed?");
				println!("[quiketc] INFO: Git may want you to add an identity, try using quiketc with -I or --ident.");
			}
		}else{
			println!("[quiketc] ERROR: git add failed! Do you have permissions? Do you have git installed?");
			println!("[quiketc] INFO: You may not have any new files, try using quiketc with -C or --noadd_commit.");
		}
	}
	else if matches.is_present("noadd_commit"){
		println!("[quiketc] INFO: Committing.");
		let output1 = Command::new("git")
			.args(&["commit", "-am","\"quiketc noadd commit\""])
			.current_dir("/etc")
			.output()
			.expect("[quiketc] ERROR: git commit failed! Do you have permissions? Do you have git installed?");
		if output1.status.success(){
			println!("[quiketc] SUCCESS: All pre-added files in /etc have been committed!");
		}else{
			println!("[quiketc] ERROR: git commit failed! Do you have permissions? Do you have git installed?");
			println!("[quiketc] INFO: You may not have any changes. If so, don't commit.");
			println!("[quiketc] INFO: Git may want you to add an identity, try using quiketc with -I or --ident.");
		}
	}
	else if matches.is_present("reset"){
		println!("[quiketc] INFO: Softresetting to last commit.");
		let output1 = Command::new("git")
			.args(&["reset", "--soft","HEAD~1"])
			.current_dir("/etc")
			.output()
			.expect("[quiketc] ERROR: softreset failed! Do you have permissions?");
		if output1.status.success(){
			println!("[quiketc] SUCCESS: Previous commit soft-resetted!");
		}else{
			println!("[quiketc] ERROR: softreset failed! Do you have permissions?");
		}
	}
	else if matches.is_present("hardreset") {
		println!("[quiketc] PROMPT: Are you sure you want to hard-reset the last commit?");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input);
		if input == "Yes\n" || input == "y\n" || input == "Y\n"|| input == "yes\n" {
			println!("[quiketc] INFO: Hardresetting to last commit.");
			let output1 = Command::new("git")
				.args(&["reset", "--hard", "HEAD~1"])
				.current_dir("/etc")
				.output()
				.expect("[quiketc] ERROR: softreset failed! Do you have permissions?");
			if output1.status.success() {
				println!("[quiketc] SUCCESS: Previous commit soft-resetted!");
			} else {
				println!("[quiketc] ERROR: softreset failed! Do you have permissions?");
			}
		} else {
			println!("[quiketc] ERROR: User rejected prompt.");
		}
	}
	else if matches.is_present("erase") {
		println!("[quiketc] PROMPT: Are you sure you want to erase your /etc repository?");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input);
		if input == "Yes\n" || input == "y\n" || input == "Y\n"|| input == "yes\n" {
			println!("[quiketc] INFO: Erasing /etc repository.");
			let output1 = Command::new("rm")
				.args(&["-rf", "/etc/.git"])
				.current_dir("/etc")
				.output()
				.expect("[quiketc] ERROR: Erasure failed! Is there a git repository?");
			if output1.status.success() {
				println!("[quiketc] SUCCESS: Erasure completed. Thanks for using quiketc!");
			} else {
				println!("[quiketc] ERROR: Erasure failed! Is there a git repository? Do you have permissions?");
			}
		} else {
			println!("[quiketc] ERROR: User rejected prompt.");
		}
	}
	else if matches.is_present("version"){
		println!("[quiketc] INFO: quiketc v2.0 by Colean!");
		println!("[quiketc] INFO: https://github.com/Zayne64/quiketc");
		println!("[quiketc] INFO: Powered by Rust!");
	}
}

