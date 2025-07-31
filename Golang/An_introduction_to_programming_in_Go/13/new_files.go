package main

import (
	"fmt"
	"io/ioutil"
)

func main(){
	bs, err := ioutil.ReadFile("test.txt")
	if err!= nil {
		return
	}
	// bs comes in part of string

	//str :=string(bs)
	fmt.Println(bs)

	str := string(bs)
	fmt.Println(str)
}

