# What is this?
A small rest API for my project [zssk-group-ticket-gen](https://github.com/ivanhrabcak/zssk-group-ticket-gen), a web application that generates documents for group tickets (which normally have to be tediously filled in).


`GET /completions/<query>`

**returns:**
```json
{
	"response": [ *completions* ]
}
```


`GET /status` (used to wake up the server on heroku)

**returns:**
```
Started!
```
