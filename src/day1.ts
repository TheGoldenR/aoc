
//@ts-ignore
import * as path from "path";
//@ts-ignore
import { readFileSync } from "fs";

export abstract class Index { 
    
    static start(): void {
        
        //@ts-ignore
        const input: string = readFileSync(path.join(process.cwd(), "../data/day1.prod"), "utf8");

        //@ts-ignore
        const max: number = input.split("\n\n").map( x => {
           return x.split("\n").map( y => parseInt(y, 10)).reduce((sum, curVal) => sum + curVal)
        }).sort( (y,z) => y-z).pop()

        console.log(max)
    }
}
