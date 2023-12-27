# Remove
Remove a scope from the user's set of permitted scopes

>Requires authorization  
>Scope: `wilford.manage`

`DELETE /api/v1/user/permitted-scopes/remove`

## Request
```json
{
    "from": "<espo user id>",
    "scope": "<scope name>"
}
```