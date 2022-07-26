from discord.ext import commands
import traceback
import discord
import random


class Events(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

        self.conf = bot.conf

    @commands.Cog.listener()
    async def on_ready(self):
        print("\n CONNECTED \n")
        await self.bot.change_presence(activity=discord.Game(name="Raid Villages"))

    @commands.Cog.listener()
    async def on_member_join(self, member):
        g_conf = self.conf.welcoming.guilds.get(str(member.guild.id))

        if g_conf is not None:
            if g_conf.welcome:
                await self.bot.get_channel(g_conf.channel).send(
                    random.choice(self.conf.welcoming.welcomes).format(member.mention)
                )

            if g_conf.role_id:
                r = member.guild.get_role(g_conf.role_id)
                if r:
                    await member.add_roles(r)

    @commands.Cog.listener()
    async def on_member_remove(self, member):
        g_conf = self.conf.welcoming.guilds.get(str(member.guild.id))

        if g_conf is not None:
            if g_conf.farewell:
                await self.bot.get_channel(g_conf.channel).send(f"Goodbye, {member.mention} ({member})")

    @commands.Cog.listener()
    async def on_member_ban(self, guild, user):
        for guild_id in self.conf.welcoming.guilds.keys():
            if guild_id != guild.id:
                g = self.bot.get_guild(int(guild_id))

                try:
                    await g.ban(user, reason=f"User was banned from {guild}")
                except Exception as e:
                    print(e)

    @commands.Cog.listener()
    async def on_message(self, m):
        if m.author.bot:
            return

        if not m.author.permissions_in(m.channel).administrator:
            content = m.content.lower()

            if "@everyone" in content and ("gift" in content or "nitro" in content or "free" in content or "steam" in content):
                await m.author.ban(reason="Advertising a scam", delete_message_days=1)
                await m.channel.send(f"{m.mention} banned lol")
                return

            for iurl in ["discord.gg/", "invite.gg/", "dsc.gg/", "dsc.lol/", "discord.com/invite/"]:
                if iurl in content:
                    await m.delete()
                    await m.channel.send(
                        embed=discord.Embed(description=f"{m.author.mention} invite links aren't allowed here.")
                    )
                    return

    @commands.Cog.listener()
    async def on_message_edit(self, m_b, m):
        if m.author.bot:
            return

        for iurl in ("discord.gg/", "invite.gg/", "dsc.gg/", "dsc.lol/", "discord.com/invite/"):
            if iurl in m.content.lower():
                if not m.author.permissions_in(m.channel).administrator:
                    await m.delete()
                    await m.channel.send(
                        embed=discord.Embed(description=f"{m.author.mention} invite links aren't allowed here.")
                    )
                    return

        g_conf = self.conf.welcoming.guilds.get(str(m.guild.id))

        if g_conf.log_channel:
            log_channel = self.bot.get_channel(g_conf.log_channel)

            embed = discord.Embed(title=f"Message Edited", url=m.jump_url)
            embed.add_field(name="Author", value=m_b.author.mention)
            embed.add_field(name="\uFEFF", value="\uFEFF")
            embed.add_field(name="Channel", value=m_b.channel.mention)

            if m_b.content:
                embed.add_field(name="Original Content", value=m_b.content[:1024], inline=False)
                if len(m_b.content) > 1024:
                    content = m_b.content[1024:]
                    for chunk in [content[i : i + 1024] for i in range(0, len(content), 1024)]:
                        embed.add_field(name="Original Content (Continued)", value=chunk, inline=False)

            if m_b.attachments:
                embed.add_field(name="Attachments", value="\n".join([a.url for a in m_b.attachments]), inline=False)

            await log_channel.send(embed=embed)

    @commands.Cog.listener()
    async def on_message_delete(self, m):
        g_conf = self.conf.welcoming.guilds.get(str(m.guild.id))

        if g_conf.log_channel:
            log_channel = self.bot.get_channel(g_conf.log_channel)

            embed = discord.Embed(title=f"Message Deleted", url=m.jump_url)
            embed.add_field(name="Author", value=m.author.mention)
            embed.add_field(name="\uFEFF", value="\uFEFF")
            embed.add_field(name="Channel", value=m.channel.mention)

            if m.content:
                embed.add_field(name="Content", value=m.content[:1024], inline=False)
                if len(m.content) > 1024:
                    content = m.content[1024:]
                    for chunk in [content[i : i + 1024] for i in range(0, len(content), 1024)]:
                        embed.add_field(name="Content (Continued)", value=chunk, inline=False)

            if m.attachments:
                embed.add_field(name="Attachments", value="\n".join([a.url for a in m.attachments]), inline=False)

            await log_channel.send(embed=embed)

    @commands.Cog.listener()
    async def on_command_error(self, ctx, e):
        for e_type in (
            commands.CommandNotFound,
            commands.NotOwner,
            discord.errors.Forbidden,
        ):
            if isinstance(e, e_type) or isinstance(e.__dict__.get("original"), e_type):
                return

        if isinstance(e, commands.CommandOnCooldown):
            seconds = round(e.retry_after, 2)

            if seconds <= 0.05:
                await ctx.reinvoke()
                return

            hours = int(seconds / 3600)
            minutes = int(seconds / 60) % 60
            seconds -= round((hours * 60 * 60) + (minutes * 60), 2)

            time = ""
            if hours > 0:
                time += f"{hours} hours, "
            if minutes > 0:
                time += f"{minutes} minutes, "
            time += f"{round(seconds, 2)} seconds"

            await ctx.send(f"Be patient and wait {time}")
        elif isinstance(e, commands.NoPrivateMessage):
            await ctx.send("Can't use that command here idiot")
        elif isinstance(e, commands.MissingPermissions):
            await ctx.send("I dare you to again motherfucker *stares motherfuckerly*")
        elif isinstance(e, commands.BotMissingPermissions):
            await ctx.send("Imagine not giving me admin perms??")
        elif isinstance(e, commands.MaxConcurrencyReached):
            await ctx.send("Try again in a bit ding dong")
        elif isinstance(e, commands.MissingRequiredArgument):
            await ctx.send("learn how to use the command right, idiot")
        elif (
            isinstance(e, commands.BadArgument)
            or isinstance(e, commands.errors.ExpectedClosingQuoteError)
            or isinstance(e, commands.errors.BadUnionArgument)
        ):
            await ctx.send("YOU FUCKING TYPED SOMETHING WRONG USE THE COMMAND RIGHT DEGENERATE")
        else:
            traceback_text = "".join(traceback.format_exception(type(e), e, e.__traceback__, 4))
            final = f"{ctx.author}: {ctx.message.content}\n\n{traceback_text}".replace("``", "\`\`\`")
            await ctx.send(f"```{final[:2000 - 6]}```")


def setup(bot):
    bot.add_cog(Events(bot))
