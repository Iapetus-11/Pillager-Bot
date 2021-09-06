import std/[json, asyncdispatch]
import dimscord, dimscmd

let
    config = parseJson(readFile("config.json"))
    client = newDiscordClient(config["token"].getStr())

var cmd = newHandler(client)

proc messageCreate(s: Shard, msg: Message) {.event(client).} =
    let handledCommand = await cmd.handleMessage(config["prefix"].getStr(), s, msg)

    if not handledCommand:
        echo msg.content

cmd.addChat("hello") do ():
    discard await client.api.sendMessage(msg.channelID, "Hello!")

waitFor client.startSession()
