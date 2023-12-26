# Info
Get information about the authorized user.

>Requires authorization

`GET /api/v1/user/info`

## Response
```jsonc
{
    "name": "<name of the user>",
    "is_admin": false // Whether the user is an EspoCRM admin
}
```