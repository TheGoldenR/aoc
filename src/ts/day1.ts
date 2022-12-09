
//@ts-ignore
import * as path from "path";
//@ts-ignore
import { readFileSync } from "fs";
        
//@ts-ignore
const input: string = readFileSync(path.join(__dirname, "../../data/day1.prod"), "utf8");

let list: number[] = input.split("\n\n")
            .map(x => x.split("\n")
                 .map(x => {  
                     let numb: number = parseInt(x);
                     return !isNaN(numb)? numb : 0;
                 }).reduce((sum: number, curVal: number) => sum + curVal, 0)
                ).sort((x,y) => x-y);

console.log("=== Part 1 ===");

console.log(list.slice(-1));

console.log("=== Part 2 ===");
console.log(list.slice(-3).reduce((sum, val)=>sum+val, 0));
