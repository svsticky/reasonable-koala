# APIs
Besides OAuth2 endpoints, Wilford also has some other APIs

## Authorization
Some endpoints require authorization.
The OAuth2 access token should be provided in the `Authorization` HTTP header, like so:
```
Authorization: Bearer <your access token>
```

To obtain an access token, refer to [OAuth2 authorization](../oauth2/authorization.md)

## CAT Authorization
CAT, or Constant-Access-Token authorization, is a special form of access tokens used by some endpoints.
These tokens are created manually and never expire (but can be revoked).

This token should be provided in the `Authorization` HTTP header, like so:
```
Authorization: Bearer <CAT token>
```