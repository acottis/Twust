

$test = Invoke-RestMethod `
-Uri 'https://id.twitch.tv/oauth2/validate' `
-Headers @{
    'Authorization' = "OAuth $($auth.access_token)"
}
# STEP 1 MANUAL TOKEN


# STEP 2 USE AUTH CODE TO GET TOKEN

$body = @{ 
    client_id='h4kgvznaowtw8lbg3ktr7cny4sqlmw'
    client_secret='elb9f55z4kkpe1xpfutgsvskwwjl7p'
    code='qda42rtinhq41oexcl0w3kjd1sw66d'
    grant_type='authorization_code'
    redirect_uri='http://localhost:8080'
}
$token = Invoke-RestMethod `
    -Uri 'https://id.twitch.tv/oauth2/token' `
    -Body $body `
    -Method POST

# Step 3 REFRESH
$body = @{ 
    client_id='h4kgvznaowtw8lbg3ktr7cny4sqlmw'
    client_secret='elb9f55z4kkpe1xpfutgsvskwwjl7p'
    grant_type='refresh_token'
    refresh_token=$token.refresh_token
}
$refresh = Invoke-RestMethod `
    -Uri 'https://id.twitch.tv/oauth2/token' `
    -Body $body `
    -Method POST

$refresh.access_token # THIS IS MY PASSWORD
$refresh.refresh_token # This is the refresh token











$test = Invoke-RestMethod -Method GET `
    -Uri 'https://api.twitch.tv/helix/users?login=twitchdev' `
    -Headers @{
        'Authorization' = "Bearer $($auth.access_token)"
        'Client-Id' = "$($body.client_id)"
    }

$res = Invoke-RestMethod -Method Get `
    -Uri "https://api.guildwars2.com/v2/characters/Azphel%20Umbruh/core"`
    -Headers @{
        'Authorization' = "Bearer $($api_key)"
    }

$res = Invoke-RestMethod -Method Get `
    -Uri "https://api.guildwars2.com/v2/pvp/standings" `
    -Headers @{
        'Authorization' = "Bearer $($api_key)"
    }

$res = Invoke-RestMethod -Method Get `
    -Uri "https://api.guildwars2.com/v2/maps" `
    -Headers @{
        'Authorization' = "Bearer $($api_key)"
    }