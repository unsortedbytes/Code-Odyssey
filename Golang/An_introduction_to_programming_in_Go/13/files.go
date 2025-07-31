package main

import (
	"fmt"
	"os"
)

func main(){
	file, err := os.Open("test.txt")
	if err != nil{
		// Handle error here
		return
	}
	defer file.Close()

	stat,err := file.Stat()
	if err != nil {
		// stat error hangling
		return 
	}

	// reading the file
	bs := make([]byte, stat.Size())
	_, err = file.Read(bs)
	if err != nil {
		// no change
		return
	}

	str :=string(bs)
	fmt.Println(str)
}
