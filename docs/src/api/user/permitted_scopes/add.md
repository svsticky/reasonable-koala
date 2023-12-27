# Add
Add a scope to the user's set of authorized scopes

>Requires authorization  
>Scope: `wilford.manage`

`POST /api/v1/user/permitted-scopes/add`

## Request
```json
{
    "to": "<espo user id>",
    "scope": "<scope name>"
}
```