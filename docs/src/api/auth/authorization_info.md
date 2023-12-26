# Authorization Info
Get information about a pending authorization

`GET /api/v1/auth/authorization-info`

## Query
```
authorization = <string : id of pending authorization>
```

## Response
```json
{
    "client_name": "<name of client that initiated the authorization>",
    "scopes": "<requested scopes>"
}
```