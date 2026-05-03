// const {sum, greet} = require("./trs.node")
import {mkdirSync} from 'fs'
import {readdir} from 'fs/promises'
import "dotenv/config"
import chalk from "chalk"
import { Command } from "commander"
import cliProgress from "cli-progress"
import boxen from "boxen"
import { Impit } from "impit"
import ora from "ora"
import { input, select } from "@inquirer/prompts"



// console.log(sum(1,2))
// console.log(greet("Izunzun"))
const program = new Command()
    .name("trs")
    .description("TRS ENGINE")
    .version("0.1.0")


const impit = new Impit({
    browser: "firefox",
})
const URL = "https://osu.ppy.sh/beatmapsets/"
const DIR = "./maps"
if(!(await Bun.file(DIR).exists())){
    mkdirSync(DIR, {recursive: true})
}
type beatmap = {
    name? : string,
    author? : string,
    id : string,
}
type counter = {
    all: number,
    succ: number,
    size: number,
}
const downloader = async (bms : beatmap[]) => {
    let cnt : counter = {succ : 0, all : bms.length, size : 0}
    const cooldowntime = process.env.COOL? +process.env.COOL : null;
    if(!cooldowntime) throw new Error(`missing cooldown time`)
    for(const bm of bms){
        const url = `https://osu.ppy.sh/beatmapsets/${bm.id}/download`
        const spinner = ora(bm.name && bm.author
            ? `pulling ${chalk.blue(bm.author)} -  ${chalk.yellow(bm.name)}`
            : `fetching ${chalk.green(url)}`) .start()

        const response = await impit.fetch(url)

        if(!response.ok) {spinner.fail("chalk.red(DitmeLoicailon)") ; throw new Error(`HTTP ${response.status}`)}

        const buffer = await response.arrayBuffer()

        const filename = url.split("beatmapsets/")[1]?.split("/")[0]

        await Bun.write(`${DIR}/${filename}`, buffer)

        spinner.succeed(`${chalk.green(bm.id)} ${chalk.blue(bm.author)} - ${chalk.yellow(bm.name)}`
            + ` ${chalk.cyan((buffer.byteLength / 1024).toFixed(2) + 'kb')}`)
        cnt.size += buffer.byteLength
        cnt.succ ++;
        await new Promise(r => setTimeout(r, 3000))
    }
    console.log(boxen(`pulled ${cnt.succ}/${cnt.all} maps\ncost ${chalk.cyan((cnt.size / 1024).toFixed(2) + ' kb')}`, {padding : 1, borderStyle: 'round', borderColor: 'magentaBright'}))
}
program
    .command("pull")
    .description("beatmap downloading")
    .action(async () => {
        const path = await input({message: "path to your import file", default: "./output.txt"})
        // const OSU_SESSION = process.env.OSU_SESSION as string
        const data = await  Bun.file(path).text() as string;
        const bms = data.split('\n').filter(l => l.trim()).map((line : string) => {
            const match = line.match(/^(\d+)\s+(.+?)\s+by:\s+(.+)$/)
            // console.log(line)
            // console.log(match)
            if(!match) return null
            // console.log(`${match[1]} ${match[2]} ${match[3]}]`)
            if(match[1] === null || match[1] === undefined) return null
            return {
                id: match[1],
                name: match[2],
                author: match[3],
            }
        }).filter((b) => b !== null)
        downloader(bms)
    })

program
    .command("init")
    .description("nhin la hieu deo phai hoi")
    .action(async () =>{
        // const session = await input({message: "your OSU_SESSION", default: process.env.OSU_SESSION || "not yet set"})
        const dir = await input({message: "where're your beatmap folder? just copy path address on your file explorer", default: process.env.DIR})
        const cooldown = await input({message: "cooldown time (3-10s)", default: "3"})
        const dereCheck = await select({message: "You such a Tsundere?",
            choices: [
                {name: "yes", value: "y"},
                {name: "No", value: "n"},
            ]})
        if(dereCheck === "y"){
            console.log("GGS")
            console.log(boxen(`your bms  will be saved at ${chalk.yellow(DIR)}`, {padding:1 , borderStyle: 'round', borderColor: 'cyan'}))
        }else{
            console.log("KYFS")
            console.log(boxen(`your bms be saved at ${DIR}`, {padding : 1, borderStyle: 'round'}))
        }
        Bun.write(".env", `COOL=${cooldown}\nDIR=${dir}`)
    })

program
    .command("push")
    .description("export portable file!")
    .action(async () => {
        const dir = process.env.DIR || "./"
        const files = await readdir(dir)

        const output = files.map(file => {
            const code = file.split(" ")[0]
            const author =file.split(" ")[1]
            const name = file.split(" - ")[1]
            console.log(`${chalk.green(code)} ${chalk.yellow(name)} by: ${chalk.blue(author)}\n`)
            return `${code} ${name} by: ${author}`
        })
        Bun.write('output.txt', output.join('\n'))
    })
program.parse()
