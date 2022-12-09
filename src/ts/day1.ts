
//@ts-ignore
import * as path from "path";
//@ts-ignore
import { readFileSync } from "fs";
        
//@ts-ignore
const input: string = readFileSync(path.join(__dirname, "../../data/day1.prod"), "utf8");

//@ts-ignore
console.log(input);

//@ts-ignore
const max: number = input.split("\n\n").map( x => {
    return x.split("\n").map( y => parseInt(y, 10)).reduce((sum, curVal) => sum + curVal)
}).sort( (y,z) => y-z).pop()

console.log(max)
