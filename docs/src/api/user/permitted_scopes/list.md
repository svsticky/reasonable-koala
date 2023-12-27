# List
List a user's permitted scopes

>Requires authorization  
>Scope: `wilford.manage`

`GET /api/v1/user/permitted-scopes/list`

## Query
```
user = <espo user id>
```

## Response
```json
{
    "scope": [
        "<scope name>"
    ]
}
```