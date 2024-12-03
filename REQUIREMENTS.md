# Requirements

## Accounts
Users must be able to:
- **Sign Up**:
  - Create a unique username and passphrase (minimum 16 characters with at least one uppercase letter, one lowercase letter, one number, and one special character).
  - Set up at least two different MFA methods (U2F, TOTP, email, recovery phrase). Users may use only U2F keys if they have two or more registered.
- **Manage Account**:
  - Update profile information: name, username, passphrase, MFA methods (subject to MFA minimums), profile picture, and preferred color mode (light, gray, dark, high-contrast).
- **Delete Account**:
  - Permanently delete their account and all profile data. Public map entries will remain, marked as "[Deleted Account]". The process must complete within 48 hours and is irreversible.
- **Interact with Maps**:
  - View a map of Bartlesville's recycling drop-off sites (using OpenStreetMap data) and their details, even when not logged in.
  - When logged in:
    - Submit new map entries.
    - Review and up/downvote other users' entries.
    - Report fraudulent entries.
    - Suggest edits to existing entries (treated like new entries in the review process).
  - Users with three or more fraud flags on their posts within a month will be banned from posting for three months.

## Security
- **Authentication**:
  - All authentication data must be protected with post-quantum encryption, in addition to TLS.
- **MFA Enforcement**:
  - MFA options must support recovery and resilience against loss of individual devices.
- **General Security**:
  - Align with OWASP best practices for web application security.

## Database
- Use MariaDB
- Regularly encrypt backups and ensure they align with the database's encryption strategy.

## Testing
This project will follow:
- **Test-Driven Development (TDD)**:
  - Write tests for all new features before implementation.
  - Cover critical components such as authentication, data validation, and scalability.
- **Rugged Principles**:
  - Conduct regular security audits and penetration tests.
  - Implement comprehensive logging to support incident response.

## Accessibility
- Provide keyboard navigation and screen reader support.
- Offer a high-contrast mode for improved visibility.

## OpenStreetMap Compliance
- Attribute OpenStreetMap as required.
- Ensure compliance with OpenStreetMap's data use policy.
- Regularly update map data to reflect changes.

## Community Moderation
- Flagged posts and fraudulent entries will be reviewed manually by moderators.
- Abuse of the reporting system may result in account restrictions.

## Data Retention
- Retain flagged posts and banned user data for 90 days for audit and appeal purposes.

## Deployment
- Deploy using a secure cloud provider.
- Utilize containerization for scalability and isolation.

