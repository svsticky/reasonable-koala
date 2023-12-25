# OAuth2 Authorization
Logging in using Wilford OAuth2 authentication.

## Quick outline
The following steps give a global outline of the OAuth2 login process
1. Your application redirects the resource owner to Wilford (Authorization step)
2. Resource owner logs in with Wilford
2. Resourec owner is redirected to your client
3. Your client exchanges the authorization code for an access token and refresh token


## Authorization
Redirect the resource owner to `/api/oauth/authorize` with the following query parameters (`application/x-www-form-urlencoded`):
```
response_type: code
client_id: <Your client's ID>
redirect_uri: <Redirect URI configured for your client>
scope: <Optional, scopes>
state: <Optional, state>
```
The state parameter will be given back to your after the authorization, unmodified.

#### Success
1. The resource owner will be redirected to Wilford's login page, where they must log in using their EspoCRM credentials.
2. The resource owner will be asked to grant your client access
3. The resource owner will be redirected to your `redirect_uri`. The `scope` query parameter will contain your authorization grant. The `state` parameter contains the `state` you provided earlier (Optional).

#### Error
1. The resource owner will be redirected to your `redirect_uri`. The `error` query parameter will contain the error.
The `error` parameter will contain a value as described per [RFC6749 Section 4.1.2.1](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1)

## Token exchange
If the previous step went successfull, your client can now exchange the authorization grant for an access- and refresh token. The `code` is contained in the query parameters, as described in the previous step.

Your client should send a `POST` request to `/api/oauth/token` with the following query parameters (`application/x-www-form-urlencoded`):
```
grant_type: code
code: <Your authorization grant>
redirect_uri: <Your client's redirect URI>
client_id: <Your client's ID>
```

#### Success
```json
{
    "access_token": "",
    "token_type": "bearer",
    "expires_in": 3600,
    "refresh_token": "",
    "scope": "",
}
```
Your application can now use the `access_token` to communicate with resource servers.

#### Error
```json
{
    "error": "<The error>"
}
```
The value of `error` is described in [RFC6749 Section 5.2](https://datatracker.ietf.org/doc/html/rfc6749#section-5.2)