# Requirements


## Accounts
Users must be able to:
* Sign up for an account by creating a unique username, passphrase, as well as setting up at least 2 different MFA methods (U2F, TOTP, email, recovery phrase). For the MFA options, there must be at least 2 different types (e.g. you cannot set only two TOTP secrets as your entire MFA setup), but an exception is made for U2F Keys: If a user has only 2 or more U2F keys on their MFA list, then that will be allowed. 
* Delete their account and all their profile data (public map item entries will remain but will be designated as having been posted by "[Deleted Account]"). Deleting one's account must be completed within 48hours and is irreversible. 
* Manage their account by updating any of the following: name, username, passphrase, MFA methods (subject to minimum requirements for MFA specified for the account creation process), profile picture, preferred color mode (light/gray/dark/high-contrast). 
* View a map (using a copy of the Bartlesville OpenStreetMap data in compliance with OpenStreetMap's terms) of Bartlesville's recycling dropoff sites, along with those site's details, even if not logged-in. 
* While logged-in, submit new map item entries, review other users' entries, up/down vote other users' entries, report entries as fraudulent, submit edits to existing entries. (Edits to existing entries are treated the same way a new entry gets treated for the community review process). 
* Users that get 3 or more fraud flags on their posts within a month will be banned from posting for 3 months. 

## Security
All authentication-related data, and all non-public user profile data, must be protected with post-quantum encryption (in addition to the presumably pre-quantum TLS which the app will have when deployed). 


## Database
This app must use MariaDB with its TDE feature. 

## Testing

This project aspires to align with both test-driven development (TDD) as well as the "Rugged" software principles: 
