package main

import (
	"os"
)

func main(){
	file, err := os.Create("2nd.txt")
	if err != nil {
		// handle the error 
		return
	}
	defer file.Close()

	file.WriteString("Hello Worl93837d")
}
