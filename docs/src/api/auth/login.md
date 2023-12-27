# Login
Log in with EspoCRM login credentials to authenticate for a pending authorization

`POST /api/v1/auth/login`

## Request
```json
{
    "authorization": "<id of pending authorization>",
    "username": "<espocrm username>",
    "password": "<espocrm password>",
    "totp_code": "<(Optional) espocrm TOTP code>"
}
```

## Response
```jsonc
{
    "status": false, // Indicates if the login was successful
    "totp_required": false, // Indicates if TOTP is required
}
```

A login is successful if `status == true`. The request should be retried with a `totp_code` if `totp_required == true`.
If `!status && !totp_required`, the credentials were likely invalid.

### Failure
The server will respond with a `403 Forbidden` if the authorization requests scopes that aren't permitted.
An authorization may always request the OIDC scopes: `openid profile email`.
