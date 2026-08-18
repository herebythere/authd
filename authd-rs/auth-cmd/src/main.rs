/*

The CLI for Authd as a service

authd setup (if db file non-existent)
	- Create organization: authd
	- It's an organization differentiated from other organizations
	- Create a user from cli with contact / email and password
	- Make user "internal"
		- allows them to add and delete organizations
		- users not "internal" cannot edit organizations
		- however all members of "authd" can edit other organization data
		- (Logs for ownership)

authd upkeep (keep db smol)
	- Delete stale entries
	- Delete soft-deleted entries
*/

fn main() {
	let args: Vec<String> = env::args().collect();
}
