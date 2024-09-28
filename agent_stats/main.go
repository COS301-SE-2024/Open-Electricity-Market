package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)


func main() {
	 dat, err := os.ReadFile("stats.txt")
    if err != nil {
		fmt.Println(err)
	}
    // fmt.Print(string(dat))
	count := 0
	agent := 0.0
	simulation := 0.0
	market := 0.0
	
	lines := strings.Split(string(dat),"\n");
	for index,line := range lines {
		fmt.Printf("%d: %s\n ",index,line);
		a,b,_ := strings.Cut(line," ")
		b = strings.Trim(b," ")
		b = strings.Trim(b,"%")
		fmt.Printf("a: %s b: %s\n",a,b)

	   	i, err := strconv.ParseFloat(b,64)
    	if err == nil {
    		if a == "amplify-agent-1" {
				agent += i;	
				count++;
			}
			if a == "amplify-simulation-1" {
				simulation += i;	
			}
			if a == "amplify-market-platform-1" {
				market += i;	
			}
		}

	}

	fmt.Println(agent/float64(count))

	fmt.Println(market/float64(count))

	fmt.Println(simulation/float64(count))
}
