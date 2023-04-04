# websocket test

## challenge

 - implement a websocket client for the server that handles connection, login, ping, message parsing, automatic reconnect, and channel forwarding to consumer app of client
 - for parsing, implement a serde enum that handles all of the possible message types

## client login message

 - after initial connection, client must send a login message with the shape:

```json
{
	"type": "login",
	"username": "anything"
}
```

## receiver msg types

 - login
```json
{
	"type": "login",
	"info": "some string"
}
```

 - msg zero
```json
{
	"type": "msg_zero",
	"zero_info": "some string"
}
```

 - msg one
```json
{
	"type": "msg_one",
	"one_info": "some string"
}
```

 - msg two
```json
{
	"type": "msg_two",
	"two_info": "some string"
}
```

 - error
```json
{
	"type": "error",
	"message": "some error message string"
}
```