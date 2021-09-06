import std/[json, asyncdispatch, strformat, logging, strutils, options]
import dimscord, dimscmd

const
    INVITE_URLS = ["discord.gg/", "invite.gg/", "dsc.gg/", "dsc.lol/", "discord.com/invite/"]
    STATUS_NAME = "raid villages"

let
    config = parseJson(readFile("config.json"))
    logger = newConsoleLogger(Level.lvlAll, "[$date $time] $levelname: ")
    client = newDiscordClient(config["token"].getStr())

addHandler(logger)

var cmd = newHandler(client)

proc filterMessage(s: Shard, msg: Message) {.async.} =
    let
        contentLower = msg.content.toLowerAscii()
        guild = s.cache.guilds[msg.guild_id.get()]
        channel = s.cache.guildChannels[msg.channel_id]
        perms = guild.computePerms(msg.member.get(), channel)

    for inviteUrlPre in INVITE_URLS:
        if inviteUrlPre in contentLower:
            if permAdministrator notin perms.allowed:
                await client.api.deleteMessage(msg.channel_id, msg.id, "Contained invite link.")
                discard await client.api.sendMessage(msg.channel_id, &"{@(msg.author)} invite links aren't allowed here.")
                return

proc onReady(s: Shard, r: Ready) {.event(client).} =
    await s.updateStatus(@[ActivityStatus(name: STATUS_NAME, kind: ActivityType.atPlaying)])
    info(&"Bot shard {s.id} ready!")

proc messageCreate(s: Shard, msg: Message) {.event(client).} =
    if msg.guild_id.isNone:
        return

    let handledCommand = await cmd.handleMessage(config["prefix"].getStr(), s, msg)

    if handledCommand:
        debug(&"{$msg.author}: {msg.content}")

    await filterMessage(s, msg)

proc messageUpdate(s: Shard, msg: Message, oldMessage: Option[Message], exists: bool) {.event(client).} =
    if exists:
        await filterMessage(s, msg)


waitFor client.startSession()
