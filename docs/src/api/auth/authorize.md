# Authorize
Grant (or not) a pending authorization

`GET /api/v1/auth/authorize`

## Query
```
authorization = <string : id of pending authorization>
grant = <true/false : whether to grant the authorization or not>
```

## Response
Responds with a `HTTP 302`.
The redirect redirects to the `redirect_uri` provided in the initial authorization request.
What exactly is put behind the redirection uri depends on the flow used. Refer to the [OAuth2 Authorization](../../oauth2/authorization.md) document.