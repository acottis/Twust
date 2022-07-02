# Twust

Twitch rust bot, primary focus on GW2 featurs at the moment but wil improve into irc chat mod

## Requirements

* GW2 API key all permissions
* Twitch Oauth token

## How to use

### Twitch OAUTH

https://discuss.dev.twitch.tv/t/programmatically-get-irc-compatible-oauth-token-for-bot/31316

**Must get the first one manually**
Client_id is the only param need to be changed to your twitch app.
https://id.twitch.tv/oauth2/authorize?scope=chat%3Aread+chat%3Aedit&redirect_uri=http%3A%2F%2Flocalhost%3A8080&client_id=h4kgvznaowtw8lbg3ktr7cny4sqlmw&response_type=code

Then we need to add the code to the ENV as shown below. The current refresh token will be added to env by the program and used to refresh

### Set enviroment up

```powershell
$env:GW2_API_KEY = 'YOUR API KEY'
$env:TWITCH_CODE = 'CODE GENERATED FROM ABOVE WEB REQUEST' # From previous step
$env:TWITCH_USERNAME = 'YOUR TWITCH BOT NAME' # The acc you used to generate the oauth
$env:TWITCH_CLIENT_ID='YOUR TWITCH APP ID' # Your twitch apps ID
$env:TWITCH_CLIENT_SECRET='YOUR TWITCH APP SECRET' # Your twitch apps secret
```


## TODO

* Think about state
* Implement leaflet map on a web server
* Map my coordinates