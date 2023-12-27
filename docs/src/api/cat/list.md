# List
List all CAT tokens

>Requires authorization  
>Scope: `wilford.manage`

`GET /api/v1/cat/list`

## Response
```json
{
    "tokens": [
        {
            "name": "<name of the token>",
            "token": "<the token>"
        }
    ]
}
```