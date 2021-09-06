import dimscord
import dimscmd
import json

let
    config = parseJson(readFile("config.json"))
    client = newDiscordClient(config["token"].getStr())

var 
