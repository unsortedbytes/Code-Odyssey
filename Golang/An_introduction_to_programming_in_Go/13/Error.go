package main

import (
	"errors"
	"fmt"
)

func main(){
	err := errors.New("error Message")
	if 2>1 {
		fmt.Println(err)
	}
}
