# APIs
Besides OAuth2 endpoints, Wilford also has some other APIs

## Authorization
Some endpoints require authorization.
The OAuth2 access token should be provided in the `Authorization` HTTP header, like so:
```
Authorization: Bearer <your access token>
```

To obtain an access token, refer to [OAuth2 authorization](../oauth2/authorization.md)