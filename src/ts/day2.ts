//@ts-ignore
import * as path from "path";
//@ts-ignore
import { readFileSync } from "fs";
        
//@ts-ignore
const input: string = readFileSync(path.join(__dirname, "../../data/day2.prod"), "utf8");

type Play = {
    id: string
    value: number
}

const playMap: Play[] = [
    {id: "A", value: 1},
    {id: "X", value: 1},
    {id: "B", value: 2},
    {id: "Y", value: 2},
    {id: "C", value: 3},
    {id: "Z", value: 3},
];

const WIN = 6;
const DRAW = 3;


function getPlayValue(play: string): number {
    return playMap.find((x) => x.id === play)?.value ?? 0 ;
}

function getRoundPoints(round: number[]): number {
   const roundWinner = round[0] - round[1];

   if(Math.abs(roundWinner) > 1) { 
      return roundWinner > 0 ?  WIN + round[1] : round[1];
   }
    
   return roundWinner > 0 ? round[1] : 
       roundWinner < 0 ? WIN + round[1] : DRAW + round[1];
}

console.log("=== Part 1 ===");
console.log(input
            .trim().split('\n')
            .map((str: string) => getRoundPoints(str.split(' ')
                                                 .map((curStr: string) => getPlayValue(curStr))
                                                )).reduce((sum, curVal) => sum + curVal, 0)
           );

console.log("=== Part 2 ===");
function playPoints(round: number[]): number{

    switch (round[1]){
        case 1: 
            //3 is the Z(Scissor) value 
            return round[0] - 1 < 1 ? 3 : round[0] - 1; 
        case 2: 
            return round[0] + DRAW;
        case 3: 
            // 1 is the A(Rock) value 
            return round[0] + 1 > 3 ? (1 + WIN) : (round[0] + 1 + WIN);
        default: 
            return 10000
    }
}

console.log(input
            .trim().split('\n')
            .map((str: string) => playPoints(str.split(' ')
                                             .map((curStr: string) => getPlayValue(curStr))
                                            )).reduce((sum, curVal) => sum + curVal, 0)
           );
