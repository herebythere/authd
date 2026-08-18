## API

Inital setup
- create admin with password

Web interface:
How maintainers manage the authd service
(Someting like 127.0.0.1:3050)
- Admin login
- Create API key (with expiration date)
- View organizations (view org id)
- View members of organizaiton
- Suspend ban or block members
- View Delete sessions
- Set global ratelimits (per organization)?
(Stretch)
- create / invite / delete patron

Web api (requires API key)
How organizations access the authd service.
Returns an answer via status code OR a one-line body response
- Create invite for organization (includes contact, possible email to send)
- Create person for organization with invite (dangerous action with "invite" category)
- Create person for organization with initial password (old school, can reset password on first login)
- Log person in and create session
- Rate limit ip session
- Rate limit logins
- Rate limit session
- Rate limit 404s (crawlers)
- Set ratelimits
- IP icebox
- Person icebox

Constants?
- there are lots of "kinds" but really it's just a string
- could there be a series of constants?

Dangerous actions:
- request add contact
- request reset password
- request add / delete account (signup delete)
- request create / delete TOTP

Server maintenance:
- Delete stale sessions
- Delete stale dangerous actions
- Delete stale api keys
