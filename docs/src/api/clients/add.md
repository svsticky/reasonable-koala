# Add
Add a new OAuth2 client

>Requires authorization  
>Scope: `wilford.manage`

`POST /api/v1/clients/add`

## Request
```json
{
    "name": "<name of the client>",
    "redirect_uri": "<redirect uri>"
}
```